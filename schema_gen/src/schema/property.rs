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
}
