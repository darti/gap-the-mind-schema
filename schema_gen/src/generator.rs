use crate::utils::escape;
use crate::{Definition, Schema};
use codegen::Scope;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

struct Entity {
    name: String,
    is_enum: bool,
}

pub struct Generator<'a> {
    schema: &'a Schema,
    structs: Rc<RefCell<HashMap<String, Entity>>>,
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

    pub fn generate(&mut self) {
        for d in &self.schema.graph {
            if d.is_primitive_type() {
                println!("{:?}", d.id);
            } else if d.is_struct_or_enum() {
                self.scope
                    .borrow_mut()
                    .new_struct(escape(d.id.as_str()).as_str());
            }
        }

        let mut scope = self.scope.borrow_mut();
    }
}

impl<'a> Display for Generator<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.scope.borrow_mut().to_string())
    }
}
