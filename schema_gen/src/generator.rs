use crate::utils::escape;
use crate::{DefType, Definition, Schema};
use codegen::Scope;
use convert_case::{Case, Casing};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Weak};

use std::rc::Rc;

struct Entity {
    name: String,
    comment: String,
    is_enum: bool,
    children: Rc<RefCell<Vec<Weak<Entity>>>>,
    members: Rc<RefCell<Vec<String>>>,
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
}

fn gen_field(_p: &Definition) {
    /*let mut field = codegen::Field::new(&def.fn_name(), self.return_type(&def.range));
    field.doc(vec![&def.doc()]);

    let mut structs = self.structs.borrow_mut();

    let mut add_field = |r: &Reference| {
        structs.get_mut(&r.id).map(|t| t.push_field(field.clone()));
    };

    match &def.domain {
        Some(Cardinality::Single(r)) => add_field(r),
        Some(Cardinality::Sequence(v)) => v.iter().for_each(add_field),
        _ => (),
    };*/
}

pub fn return_type(ty: &Vec<String>) -> String {
    /* match ty {
        Some(Cardinality::Single(r)) => actual_type(&r.id),
        Some(Cardinality::Sequence(v)) => {
            let ss: Vec<String> = v.iter().map(|r| actual_type(&r.id)).collect();

            ss.join(",")
        }
        None => "()".to_string(),
    }*/

    "()".to_string()
}

pub fn generate(schema: &Schema) -> String {
    let mut scope = Scope::new();
    let mut structs: HashMap<String, Entity> = HashMap::default();

    let get_parent_struct = |d: &Definition| {};

    for df in schema.graph.iter().map(|d| DefType::from(d)) {
        match df {
            DefType::Primitive(d) => {}
            DefType::Property(d) => {
                let parent = escape(d.ty.into_iter().next().unwrap());
                let e = structs
                    .entry(parent.clone())
                    .or_insert_with(|| Entity::new(&parent));

                e.members
                    .borrow_mut()
                    .push(escape(d.label.to_string().as_ref()));

                for t in d.parent_types() {
                    //e.parents.borrow_mut().push(t.to_string());
                }
            }
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
                e.members.borrow_mut().push(d.label.to_string());
            }
        }
    }

    for (n, e) in structs {
        if e.is_enum {
            let en = scope.new_enum(&n);

            for m in e.members.borrow().iter() {
                en.new_variant(m);
            }
        } else {
            let st = scope.new_struct(&n);

            for m in e.members.borrow().iter() {
                st.field(m.to_case(Case::Snake).as_str(), "()");
            }
        }
    }

    scope.to_string()
}
