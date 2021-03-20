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
fn generate_property(structs: &mut HashMap<String, Entity>, d: &Definition) {
    let label = escape(d.label.to_string().as_ref());

    let (mut simple, mut complex): (Vec<_>, Vec<&str>) =
        d.ranges().partition_map(|r: &str| match simple_type(r) {
            Some(s) => Either::Left(s),
            None => Either::Right(r),
        });

    simple.sort_unstable();

    let mut hash = DefaultHasher::new();
    simple.hash(&mut hash);
    let hash = hash.finish().to_string();

    let enum_name = if simple.len() == 1 {
        simple.first().unwrap().to_string()
    } else {
        let n = simple.join("Or");

        structs.entry(hash.clone()).or_insert_with(|| {
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
        let parent = structs
            .entry(parent.clone())
            .or_insert_with(|| Entity::new(&parent));

        parent
            .members
            .borrow_mut()
            .push((label.clone(), enum_name.to_string()));
    }
}

pub fn generate(schema: &Schema) -> String {
    let mut scope = Scope::new();
    let mut structs: HashMap<String, Entity> = HashMap::default();

    for df in schema.graph.iter().map(|d| DefType::from(d)) {
        match df {
            DefType::Primitive(d) => {}
            DefType::Property(d) => generate_property(&mut structs, d),
            DefType::StructEnum(d) => {
                let name = escape(d.label.to_string().as_str());
                let e = structs
                    .entry(name.clone())
                    .or_insert_with(|| Entity::new(&name));

                if let Some(c) = &d.comment {
                    e.comment.push_str(c.to_string().as_str())
                }
            }
            DefType::EnumMember(d) => {
                let parent = escape(d.ty.into_iter().next().unwrap());
                let mut e = structs
                    .entry(parent.clone())
                    .or_insert_with(|| Entity::new(&parent));

                e.is_enum = true;
                e.members
                    .borrow_mut()
                    .push((d.label.to_string(), "".to_string()));
            }
        }
    }

    for (n, e) in structs {
        if e.is_enum {
            let en = scope.new_enum(&e.name);

            for (m, t) in e.members.borrow().iter() {
                en.new_variant(m);
            }
        } else {
            let st = scope.new_struct(&e.name);

            for (m, t) in e.members.borrow().iter() {
                st.field(m.to_case(Case::Snake).as_str(), t);
            }
        }
    }

    scope.import("url", "Url");
    scope.import("chrono", "{Date, DateTime}");

    scope.to_string()
}
