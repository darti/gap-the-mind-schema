use crate::utils::escape;
use crate::{DefType, Definition, Schema};
use codegen::{Enum, Field, Scope, Struct};
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
    connections: Vec<Prop>,
}

impl Entity {
    fn new(name: &str) -> Self {
        Entity {
            name: name.to_string(),
            doc: String::default(),
            props: Vec::new(),
            connections: Vec::new(),
        }
    }

    fn add_prop(&mut self, name: &str, doc: &Option<String>, ty: &str) {
        self.props.push(Prop {
            name: name.to_string(),
            doc: doc.as_deref().unwrap_or("tot").to_string(),
            ty: ty.to_string(),
        })
    }

    fn add_connection(&mut self, name: &str, doc: &Option<String>, ty: &str) {
        self.connections.push(Prop {
            name: name.to_string(),
            doc: doc.as_deref().unwrap_or("").to_string(),
            ty: ty.to_string(),
        })
    }

    fn to_struct<'a>(&self, scope: &'a mut Scope) -> &'a mut Struct {
        let s = scope.new_struct(&self.name).vis("pub");
        s.doc(&self.doc);

        for p in &self.props {
            let mut f = Field::new(&p.name, &p.ty);
            f.doc(p.doc.lines().collect());
            s.push_field(f);
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
    objects: HashMap<String, Entity>,
    unions: HashMap<String, Entity>,
}

impl Generation {
    pub fn new() -> Self {
        Generation {
            objects: HashMap::default(),
            unions: HashMap::default(),
        }
    }

    fn get_object(&mut self, name: &str) -> &mut Entity {
        self.objects
            .entry(name.to_string())
            .or_insert_with(|| Entity::new(name))
    }

    fn get_union(&mut self, types: &Vec<(&str, &str)>) -> String {
        if types.is_empty() {
            String::from("()")
        } else if types.len() == 1 {
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
                DefType::Object(d) => {
                    let name = escape(d.label.to_string().as_str());
                    let mut e = self.get_object(&name);

                    if let Some(c) = d.doc() {
                        e.doc = c;
                    }
                }
            }
        }
    }

    fn create_property(&mut self, d: &Definition) {
        let label = escape(d.label.to_string().as_ref());

        let (mut simple, mut complex): (Vec<_>, Vec<_>) =
            d.ranges().partition_map(|r: &str| match simple_type(r) {
                Some(s) => Either::Left(s),
                None => {
                    let t = escape(r);
                    Either::Right((t.clone(), t))
                }
            });

        simple.sort_unstable();
        complex.sort_unstable();

        let complex: Vec<(&str, &str)> = complex
            .iter()
            .map(|(s, t)| (s.as_ref(), t.as_ref()))
            .collect();

        let simple_ty = self.get_union(&simple);
        let complex_ty = self.get_union(&complex);

        let has_simple = !simple.is_empty();
        let has_complex = !complex.is_empty();

        for t in d.domains() {
            let target = escape(t);
            let target = self.get_object(&target);

            if has_simple {
                target.add_prop(&label, &d.doc(), &simple_ty);
            }

            if has_complex {
                target.add_connection(&label, &d.doc(), &complex_ty);
            }
        }
    }

    pub fn generate_structs(&self) -> String {
        let mut scope = Scope::new();

        scope.import("url", "Url");
        scope.import("chrono", "{Date, DateTime, NaiveTime, Utc}");
        scope.import("crate::enums", "*");
        scope.import("crate::unions", "*");

        for (_n, e) in &self.objects {
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

        // for (_n, e) in &self.enums {
        //     e.to_enum(&mut scope);
        // }

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
