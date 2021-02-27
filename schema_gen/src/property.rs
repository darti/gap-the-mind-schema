use crate::{Cardinality, Reference, Text};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Property {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "rdfs:label")]
    label: Text,

    #[serde(rename = "rdfs:comment")]
    comment: Option<Text>,

    #[serde(rename = "schema:domainIncludes")]
    domain: Option<Cardinality<Reference>>,

    #[serde(rename = "schema:rangeIncludes")]
    range: Option<Cardinality<Reference>>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}
