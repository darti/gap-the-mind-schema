use crate::utils::escape;
use crate::{DefType, Definition, Schema};
use codegen::Scope;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

struct Entity {
    name: String,
    comment: String,
    is_enum: bool,
    members: Rc<RefCell<Vec<String>>>,
}

impl Entity {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            comment: String::default(),
            is_enum: false,
            members: Rc::new(RefCell::new((Vec::new()))),
        }
    }
}

pub struct Generator {}

impl Generator {
    fn gen_struct(&mut self, c: &Definition) {
        //self.structs.borrow_mut().insert(c.id.clone(), c.generate());
    }

    fn gen_field(&self, _p: &Definition) {
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

    pub fn return_type(&self, ty: &Vec<String>) -> String {
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
}

pub fn generate(schema: &Schema) -> String {
    let mut scope = Scope::new();
    let mut structs: HashMap<String, Entity> = HashMap::default();

    for df in schema.graph.iter().map(|d| DefType::from(d)) {
        match df {
            DefType::Primitive(d) => {}
            DefType::Property(d) => {}
            DefType::StructEnum(d) => {
                let name = escape(d.label.to_string().as_str());
                let mut e = structs
                    .entry(name.clone())
                    .or_insert_with(|| Entity::new(&name));

                if let Some(c) = &d.comment {
                    e.comment.push_str(c.to_string().as_str())
                }
            }
            DefType::EnumMember(d) => {
                let parent = d.ty.into_iter().next().unwrap();
                let parent = escape(parent);
                let mut e = structs
                    .entry(parent.clone())
                    .or_insert_with(|| Entity::new(&parent));

                e.is_enum = true;
            }
        }
    }

    for (n, e) in structs {
        if e.is_enum {
            scope.new_enum(&n);
        } else {
            scope.new_struct(&n);
        }
    }

    scope.to_string()
}
