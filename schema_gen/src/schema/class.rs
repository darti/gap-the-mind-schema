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
    parents: Option<Cardinality<Reference>>,

    #[serde(rename = "rdfs:comment")]
    comment: Option<Text>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
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

#[cfg(test)]
mod tests {
    use crate::class::Class;

    #[test]
    fn class_deserialize() {
        let doc = r###"
            {
              "@id": "schema:Muscle",
              "@type": "rdfs:Class",
              "rdfs:comment": "A muscle is an anatomical structure consisting of a contractile form of tissue that animals use to effect movement.",
              "rdfs:label": "Muscle",
              "rdfs:subClassOf": {
                "@id": "schema:AnatomicalStructure"
              },
              "schema:isPartOf": {
                "@id": "https://health-lifesci.schema.org"
              }
            }
        "###;

        let obj: Class = serde_json::from_str(&doc).unwrap();

        assert_eq!(obj.id, "schema:Muscle");
    }
}
