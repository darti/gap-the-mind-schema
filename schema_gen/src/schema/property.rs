use crate::elements::{Cardinality, Reference, Text};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Property {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@type")]
    ty: String,

    #[serde(rename = "rdfs:label")]
    label: Text,

    #[serde(rename = "rdfs:comment")]
    comment: Option<Text>,

    #[serde(rename = "schema:domainIncludes")]
    domain: Cardinality<Reference>,

    #[serde(rename = "schema:rangeIncludes")]
    range: Cardinality<Reference>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use crate::property::Property;

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

        let obj: Property = serde_json::from_str(&doc).unwrap();
    }
}
