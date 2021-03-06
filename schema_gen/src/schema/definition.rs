use crate::class::Class;
use crate::enum_member::EnumMember;
use crate::property::Property;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Other {
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Definition {
    Property(Property),
    Class(Class),
    EnumMember(EnumMember),

    Other(Other),
}

impl Definition {}

#[cfg(test)]
mod tests {
    use crate::class::Class;
    use crate::definition::Definition;

    #[test]
    fn property_deserialize() {
        let doc = r###"
             {
              "@id": "schema:healthPlanCostSharing",
              "@type": "rdf:Property",
              "rdfs:comment": "Whether The costs to the patient for services under this network or formulary.",
              "rdfs:label": "healthPlanCostSharing",
              "schema:domainIncludes": [
                {
                  "@id": "schema:HealthPlanNetwork"
                },
                {
                  "@id": "schema:HealthPlanFormulary"
                }
              ],
              "schema:isPartOf": {
                "@id": "https://pending.schema.org"
              },
              "schema:rangeIncludes": {
                "@id": "schema:Boolean"
              },
              "schema:source": {
                "@id": "https://github.com/schemaorg/schemaorg/issues/1062"
              }
            }
        "###;

        let obj: Definition = serde_json::from_str(&doc).unwrap();

        assert!(matches!(obj, Definition::Property(_)));
    }

    #[test]
    fn enum_member_deserialize() {
        let doc = r###"
           {
              "@id": "schema:Ear",
              "@type": "schema:PhysicalExam",
              "rdfs:comment": "Ear function assessment with clinical examination.",
              "rdfs:label": "Ear",
              "schema:isPartOf": {
                "@id": "https://health-lifesci.schema.org"
              }
        }
        "###;

        let obj: Definition = serde_json::from_str(&doc).unwrap();

        assert!(matches!(obj, Definition::EnumMember(_)));
    }

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

        let obj: Definition = serde_json::from_str(&doc).unwrap();

        assert!(matches!(obj, Definition::Class(_)));
    }
}
