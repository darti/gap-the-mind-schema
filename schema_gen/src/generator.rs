use crate::return_types::actual_type;
use crate::{Cardinality, Definition, Reference, Schema};
use codegen::{Scope, Struct};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub struct Generator<'a> {
    schema: &'a Schema,
    structs: Rc<RefCell<HashMap<String, Struct>>>,
    scope: Rc<RefCell<Scope>>,
}

impl<'a> Generator<'a> {
    pub fn new(schema: &'a Schema) -> Self {
        Generator {
            schema,
            scope: Rc::new(RefCell::new(Scope::new())),
            structs: Rc::new(RefCell::new(HashMap::default())),
        }
    }

    fn gen_struct(&mut self, def: &Definition) {
        let mut s = Struct::new(&def.type_name());

        s.doc(&def.doc());
        self.structs.borrow_mut().insert(def.id.clone(), s);
    }

    fn gen_field(&self, def: &Definition) {
        let mut field = codegen::Field::new(&def.fn_name(), self.return_type(&def.range));
        field.doc(vec![&def.doc()]);

        let mut structs = self.structs.borrow_mut();

        let mut add_field = |r: &Reference| {
            structs.get_mut(&r.id).map(|t| t.push_field(field.clone()));
        };

        match &def.domain {
            Some(Cardinality::Single(r)) => add_field(r),
            Some(Cardinality::Sequence(v)) => v.iter().for_each(add_field),
            _ => (),
        };
    }

    pub fn return_type(&self, ty: &Option<Cardinality<Reference>>) -> String {
        match ty {
            Some(Cardinality::Single(r)) => actual_type(&r.id),
            Some(Cardinality::Sequence(v)) => {
                let ss: Vec<String> = v.iter().map(|r| actual_type(&r.id)).collect();

                return ss.join(",");
            }
            None => "()".to_string(),
        }
    }

    pub fn generate(&mut self) {
        let class_type = String::from("rdfs:Class");
        let property_type = String::from("rdf:Property");

        self.schema
            .graph
            .iter()
            .filter(|d| match &d.typ {
                Cardinality::Single(t) => *t == class_type,
                _ => false,
            })
            .for_each(|d| self.gen_struct(d));

        self.schema
            .graph
            .iter()
            .filter(|d| match &d.typ {
                Cardinality::Single(t) => *t == property_type,
                _ => false,
            })
            .for_each(|d| self.gen_field(d));

        let mut scope = self.scope.borrow_mut();

        self.structs.borrow_mut().iter().for_each(|(k, s)| {
            scope.push_struct(s.clone());
        });
    }
}

impl<'a> Display for Generator<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.scope.borrow_mut().to_string())
    }
}
