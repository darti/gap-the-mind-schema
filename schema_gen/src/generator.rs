use crate::utils::escape;
use crate::{DefType, Definition, Schema};
use codegen::Scope;
use convert_case::{Case, Casing};
use itertools::{Either, Itertools};
use std::{cell::RefCell, hash::Hasher, rc::Weak};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hash,
};

use std::rc::Rc;

struct Entity {
    name: String,
    comment: String,
    is_enum: bool,
    children: Rc<RefCell<Vec<Weak<Entity>>>>,
    members: Rc<RefCell<Vec<(String, String)>>>,
}

impl Entity {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            comment: String::default(),
            is_enum: false,
            children: Rc::new(RefCell::new(Vec::new())),
            members: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn new_enum(name: &str) -> Self {
        Self {
            name: name.to_string(),
            comment: String::default(),
            is_enum: true,
            children: Rc::new(RefCell::new(Vec::new())),
            members: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

pub struct Generation {
    structs: HashMap<String, Entity>,
    unions: HashMap<String, Entity>,
}

impl Generation {
    pub fn new(schema: &Schema) -> Self {
        let mut gen = Generation {
            structs: HashMap::default(),
            unions: HashMap::default(),
        };

        gen.generate(schema);

        gen
    }

    pub fn generate(&mut self, schema: &Schema) {
        for df in schema.graph.iter().map(|d| DefType::from(d)) {
            match df {
                DefType::Primitive(d) => {}
                DefType::Property(d) => self.create_property(d),
                DefType::StructEnum(d) => {
                    let name = escape(d.label.to_string().as_str());
                    let e = self
                        .structs
                        .entry(name.clone())
                        .or_insert_with(|| Entity::new(&name));

                    if let Some(c) = &d.comment {
                        e.comment.push_str(c.to_string().as_str())
                    }
                }
                DefType::EnumMember(d) => {
                    let parent = escape(d.ty.into_iter().next().unwrap());
                    let mut e = self
                        .structs
                        .entry(parent.clone())
                        .or_insert_with(|| Entity::new(&parent));

                    e.is_enum = true;
                    e.members
                        .borrow_mut()
                        .push((d.label.to_string(), "".to_string()));
                }
            }
        }
    }

    fn create_property(&mut self, d: &Definition) {
        let label = escape(d.label.to_string().as_ref());

        let (mut simple, mut complex): (Vec<_>, Vec<&str>) =
            d.ranges().partition_map(|r: &str| match simple_type(r) {
                Some(s) => Either::Left(s),
                None => Either::Right(r),
            });

        simple.sort_unstable();
        let has_simple = !simple.is_empty();

        let enum_name = if simple.len() == 1 {
            simple.first().unwrap().to_string()
        } else if simple.is_empty() {
            "???".to_string()
        } else {
            let mut hash = DefaultHasher::new();
            simple.hash(&mut hash);
            let hash = hash.finish().to_string();

            let n = simple.join("Or");

            self.unions.entry(hash.clone()).or_insert_with(|| {
                let e = Entity::new_enum(n.clone().as_ref());

                for s in simple {
                    e.members.borrow_mut().push((s, "".to_string()));
                }

                e
            });

            n.to_string()
        };

        for dom in d.domains() {
            let parent = escape(dom);
            let parent = self
                .structs
                .entry(parent.clone())
                .or_insert_with(|| Entity::new(&parent));

            if has_simple {
                parent
                    .members
                    .borrow_mut()
                    .push((label.clone(), enum_name.to_string()));
            };
        }
    }

    pub fn generate_structs(&self) -> String {
        let mut scope = Scope::new();

        scope.import("url", "Url");
        scope.import("chrono", "{Date, DateTime}");
        scope.import("crate::enums", "*");
        scope.import("crate::unions", "*");

        for (_n, e) in &self.structs {
            if !e.is_enum {
                let st = scope.new_struct(&e.name).vis("pub");

                for (m, t) in e.members.borrow().iter() {
                    st.field(m.to_case(Case::Snake).as_str(), t);
                }
            }
        }

        scope.to_string()
    }

    pub fn generate_enums(&self) -> String {
        let mut scope = Scope::new();

        scope.import("url", "Url");
        scope.import("chrono", "{Date, DateTime}");

        for (_n, e) in &self.structs {
            if e.is_enum {
                let en = scope.new_enum(&e.name).vis("pub");

                for (m, _t) in e.members.borrow().iter() {
                    en.new_variant(m);
                }
            }
        }

        scope.to_string()
    }

    pub fn generate_unions(&self) -> String {
        let mut scope = Scope::new();

        scope.import("url", "Url");
        scope.import("chrono", "{Date, DateTime}");

        for (_n, e) in &self.unions {
            let en = scope.new_enum(&e.name).vis("pub");

            for (m, t) in e.members.borrow().iter() {
                let name = m.to_case(Case::UpperCamel);
                let v = en.new_variant(&name);
                v.tuple(m);
            }
        }

        scope.to_string()
    }
}

fn simple_type(s: &str) -> Option<String> {
    if s == "schema:Text" {
        Some("String".to_string())
    } else if s == "schema:Integer" {
        Some("i64".to_string())
    } else if s == "schema:Float" {
        Some("f64".to_string())
    } else if s == "schema:URL" {
        Some("Url".to_string())
    } else if s == "schema:Boolean" {
        Some("bool".to_string())
    } else if s == "schema:Date" {
        Some("Date".to_string())
    } else if s == "schema:Time" {
        Some("Time".to_string())
    } else if s == "schema:DateTime" {
        Some("DateTime".to_string())
    } else {
        None
    }
}
