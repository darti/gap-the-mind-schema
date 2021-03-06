use crate::elements::{Cardinality, Reference, Text};
use codegen::Struct;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "rdfs:label")]
    label: Text,

    #[serde(rename = "rdfs:subClassOf")]
    parents: Cardinality<Reference>,

    #[serde(rename = "rdfs:comment")]
    comment: Option<Text>,
}

impl Class {
    pub fn generate(&self) -> Struct {
        let mut s = Struct::new(&self.label.escape());
        s.vis("pub");

        if let Some(c) = &self.comment {
            s.doc(&c.to_string());
        }

        s
    }
}
