use crate::utils::escape;
use crate::{DefType, Definition, Schema};
use codegen::{Enum, Scope, Struct};
use convert_case::{Case, Casing};
use itertools::{Either, Itertools};
use std::collections::HashMap;

struct Prop {
    name: String,
    doc: String,
    ty: String,
}

struct Entity {
    name: String,
    doc: String,
    props: Vec<Prop>,
}

impl Entity {
    fn new(name: &str) -> Self {
        Entity {
            name: name.to_string(),
            doc: String::default(),
            props: Vec::new(),
        }
    }

    fn add_prop(&mut self, name: &str, doc: &Option<String>, ty: &str) {
        self.props.push(Prop {
            name: name.to_string(),
            doc: doc.as_deref().unwrap_or("").to_string(),
            ty: ty.to_string(),
        })
    }

    fn to_struct<'a>(&self, scope: &'a mut Scope) -> &'a mut Struct {
        let s = scope.new_struct(&self.name).vis("pub");
        s.doc(&self.doc);

        for p in &self.props {
            s.field(&p.name, &p.ty).doc(&p.doc);
        }

        s
    }

    fn to_enum<'a>(&self, scope: &'a mut Scope) -> &'a mut Enum {
        let s = scope.new_enum(&self.name).vis("pub");
        s.doc(&self.doc);

        for p in &self.props {
            s.new_variant(&p.name);
        }

        s
    }

    fn to_union<'a>(&self, scope: &'a mut Scope) -> &'a mut Enum {
        let s = scope.new_enum(&self.name).vis("pub");
        s.doc(&self.doc);

        for p in &self.props {
            s.new_variant(&p.name).tuple(&p.ty);
        }

        s
    }
}

pub struct Generation {
    structs: HashMap<String, Entity>,
    enums: HashMap<String, Entity>,
    unions: HashMap<String, Entity>,
}

impl Generation {
    pub fn new() -> Self {
        Generation {
            structs: HashMap::default(),
            enums: HashMap::default(),
            unions: HashMap::default(),
        }
    }

    fn get_struct(&mut self, name: &str) -> &mut Entity {
        self.structs
            .entry(name.to_string())
            .or_insert_with(|| Entity::new(name))
    }

    fn get_enum(&mut self, name: &str) -> &mut Entity {
        self.enums
            .entry(name.to_string())
            .or_insert_with(|| Entity::new(name))
    }

    fn get_union(&mut self, types: &Vec<(&str, &str)>) -> String {
        if types.len() == 1 {
            types.first().unwrap().1.to_string()
        } else {
            let name = types.iter().map(|e| e.0).join("Or");
            self.unions.entry(name.clone()).or_insert_with(|| {
                let mut e = Entity::new(&name);

                for (s, t) in types {
                    e.add_prop(&s.to_case(Case::UpperCamel), &Option::None, t);
                }

                e
            });

            name
        }
    }

    pub fn generate(&mut self, schema: &Schema) {
        for df in schema.graph.iter().map(|d| DefType::from(d)) {
            match df {
                DefType::Primitive(_d) => {}
                DefType::Property(d) => self.create_property(d),
                DefType::Struct(d) => {
                    let name = escape(d.label.to_string().as_str());
                    let mut e = self.get_struct(&name);

                    if let Some(c) = d.doc() {
                        e.doc = c;
                    }
                }
                DefType::Enum(d) => {
                    let name = escape(d.label.to_string().as_str());
                    let e = self.get_enum(&name);

                    if let Some(c) = d.doc() {
                        e.doc = c;
                    }
                }
                DefType::EnumMember(d) => {
                    let parent = escape(d.ty.into_iter().next().unwrap());

                    self.get_enum(&parent)
                        .add_prop(&d.label.to_string(), &d.doc(), "");
                }
            }
        }
    }

    fn create_simple_property(
        &mut self,
        domains: Box<dyn Iterator<Item = &str> + '_>,
        name: &str,
        types: &Vec<(&str, &str)>,
        doc: &Option<String>,
    ) {
        let field_type = self.get_union(types);

        for dom in domains {
            let parent = escape(dom);
            self.get_struct(&parent).add_prop(name, doc, &field_type);
        }
    }

    fn create_complex_property(
        &mut self,
        domains: Box<dyn Iterator<Item = &str> + '_>,
        name: &str,
        types: &Vec<(&str, &str)>,
    ) {
        let field_type = self.get_union(types);
    }

    fn create_property(&mut self, d: &Definition) {
        let label = escape(d.label.to_string().as_ref());

        let (mut simple, complex): (Vec<_>, Vec<_>) =
            d.ranges().partition_map(|r: &str| match simple_type(r) {
                Some(s) => Either::Left(s),
                None => {
                    let t = escape(r);
                    Either::Right((t.clone(), t))
                }
            });

        let mut complex: Vec<(&str, &str)> = complex
            .iter()
            .map(|(s, t)| (s.as_ref(), t.as_ref()))
            .collect();

        if !simple.is_empty() {
            simple.sort_unstable();
            self.create_simple_property(d.domains(), &label, &simple, &d.doc());
        }

        if !complex.is_empty() {
            complex.sort_unstable();
            self.create_complex_property(d.domains(), &label, &complex);
        }
    }

    pub fn generate_structs(&self) -> String {
        let mut scope = Scope::new();

        scope.import("url", "Url");
        scope.import("chrono", "{Date, DateTime, NaiveTime, Utc}");
        scope.import("crate::enums", "*");
        scope.import("crate::unions", "*");

        for (_n, e) in &self.structs {
            e.to_struct(&mut scope);
        }

        scope.to_string()
    }

    pub fn generate_unions(&self) -> String {
        let mut scope = Scope::new();

        scope.import("url", "Url");
        scope.import("chrono", "{Date, DateTime, NaiveTime, Utc}");
        scope.import("crate::structs", "*");
        scope.import("crate::enums", "*");

        for (_n, e) in &self.unions {
            e.to_union(&mut scope);
        }

        scope.to_string()
    }

    pub fn generate_enums(&self) -> String {
        let mut scope = Scope::new();

        scope.import("url", "Url");
        scope.import("chrono", "{Date, DateTime, NaiveTime, Utc}");

        for (_n, e) in &self.enums {
            e.to_enum(&mut scope);
        }

        scope.to_string()
    }
}

fn simple_type(s: &str) -> Option<(&str, &str)> {
    if s == "schema:Text" {
        Some(("String", "String"))
    } else if s == "schema:Integer" {
        Some(("Integer", "i64"))
    } else if s == "schema:Float" {
        Some(("Float", "f64"))
    } else if s == "schema:Number" {
        Some(("Number", "f64"))
    } else if s == "schema:URL" {
        Some(("Url", "Url"))
    } else if s == "schema:Boolean" {
        Some(("Bool", "bool"))
    } else if s == "schema:Date" {
        Some(("Date", "Date<Utc>"))
    } else if s == "schema:Time" {
        Some(("Time", "NaiveTime"))
    } else if s == "schema:DateTime" {
        Some(("DateTime", "DateTime<Utc>"))
    } else {
        None
    }
}
