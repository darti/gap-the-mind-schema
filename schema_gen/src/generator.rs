use crate::utils::escape;
use crate::{DefType, Definition, Schema};
use codegen::Scope;
use convert_case::{Case, Casing};
use itertools::{Either, Itertools};
use std::hash::Hasher;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hash,
};

pub struct Generation {
    structs: HashMap<String, codegen::Struct>,
    enums: HashMap<String, codegen::Enum>,
    unions: HashMap<String, codegen::Enum>,
}

impl Generation {
    pub fn new(schema: &Schema) -> Self {
        let mut gen = Generation {
            structs: HashMap::default(),
            enums: HashMap::default(),
            unions: HashMap::default(),
        };

        gen.generate(schema);

        gen
    }

    fn get_enum(&mut self, name: String) -> &mut codegen::Enum {
        self.enums
            .entry(name.clone())
            .or_insert_with(|| codegen::Enum::new(&name))
    }

    fn get_union(&mut self, types: &Vec<(&str, &str)>) -> String {
        if types.len() == 1 {
            types.first().unwrap().1.to_string()
        } else {
            let mut hash = DefaultHasher::new();
            types.hash(&mut hash);
            let hash = hash.finish().to_string();

            let name = types.iter().map(|e| e.0).join("Or");
            self.unions.entry(name.clone()).or_insert_with(|| {
                let mut e = codegen::Enum::new(&name);

                for (s, t) in types {
                    let name = s.to_case(Case::UpperCamel);
                    let v = e.new_variant(&name);
                    v.tuple(t);
                }

                e
            });

            name
        }
    }

    fn get_struct(&mut self, name: String) -> &mut codegen::Struct {
        self.structs
            .entry(name.clone())
            .or_insert_with(|| codegen::Struct::new(&name))
    }

    pub fn generate(&mut self, schema: &Schema) {
        for df in schema.graph.iter().map(|d| DefType::from(d)) {
            match df {
                DefType::Primitive(d) => {}
                DefType::Property(d) => self.create_property(d),
                DefType::Struct(d) => {
                    let name = escape(d.label.to_string().as_str());
                    let e = self.get_struct(name);

                    if let Some(c) = d.doc() {
                        e.doc(&c);
                    }
                }
                DefType::Enum(d) => {
                    let name = escape(d.label.to_string().as_str());
                    let e = self.get_enum(name);

                    if let Some(c) = d.doc() {
                        e.doc(&c);
                    }
                }
                DefType::EnumMember(d) => {
                    let parent = escape(d.ty.into_iter().next().unwrap());

                    self.get_enum(parent)
                        .new_variant(d.label.to_string().as_ref());
                }
            }
        }
    }

    fn create_simple_property(
        &mut self,
        domains: Box<dyn Iterator<Item = &str> + '_>,
        label: String,
        types: &Vec<(&str, &str)>,
        doc: &Option<String>,
    ) {
        let field_type = self.get_union(types);

        for dom in domains {
            let parent = escape(dom);
            let parent = self.get_struct(parent);
            let mut f = codegen::Field::new(&label, field_type.clone());

            if let Some(d) = doc {
                f.doc(vec![d]);
            }

            parent.push_field(f);
        }
    }

    fn create_complex_property(
        &mut self,
        domains: Box<dyn Iterator<Item = &str> + '_>,
        label: String,
        types: &Vec<(&str, &str)>,
    ) {
        let field_type = self.get_union(types);
    }

    fn create_property(&mut self, d: &Definition) {
        let label = escape(d.label.to_string().as_ref());

        let (mut simple, mut complex): (Vec<_>, Vec<_>) =
            d.ranges().partition_map(|r: &str| match simple_type(r) {
                Some(s) => Either::Left(s),
                None => {
                    let t = escape(r);
                    Either::Right((t.as_ref(), t.as_ref()))
                }
            });

        if !simple.is_empty() {
            simple.sort_unstable();
            self.create_simple_property(d.domains(), label, &simple, &d.doc());
        }

        if complex.is_empty() {
            complex.sort_unstable();
            self.create_complex_property(d.domains(), label, &complex);
        }
    }

    pub fn generate_structs(&self) -> String {
        let mut scope = Scope::new();

        scope.import("url", "Url");
        scope.import("chrono", "{Date, DateTime, NaiveTime, Utc}");
        scope.import("crate::enums", "*");
        scope.import("crate::unions", "*");

        for (n, s) in &self.structs {
            scope.push_struct(*s.vis("pub"));
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
            scope.push_enum(*e.vis("pub"));
        }

        scope.to_string()
    }

    pub fn generate_enums(&mut self) -> String {
        let mut scope = Scope::new();

        scope.import("url", "Url");
        scope.import("chrono", "{Date, DateTime, NaiveTime, Utc}");

        for (_n, e) in &mut self.enums {
            e.vis("pub");

            scope.push_enum(*e);
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
