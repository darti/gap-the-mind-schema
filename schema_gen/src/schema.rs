use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use url::Url;

const DIGITS: [&str; 10] = [
    "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
];

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Cardinality<T> {
    Single(T),
    Sequence(Vec<T>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Text {
    Simple(String),
    I18n {
        #[serde(rename = "@language")]
        lang: String,
        #[serde(rename = "@value")]
        value: String,
    },
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Text::Simple(s) => write!(f, "{}", s),
            Text::I18n { lang, value } => write!(f, "{}", value),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "rdfs:label")]
    label: Text,

    #[serde(rename = "rdfs:subClassOf")]
    parents: Option<Cardinality<Reference>>,

    #[serde(rename = "rdfs:comment")]
    comment: Option<Text>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Definition {
    Class(Class),

    Property(Property),

    RdfOther {
        #[serde(rename = "@id")]
        id: String,

        #[serde(rename = "@type")]
        rdf_type: Cardinality<String>,

        #[serde(rename = "rdfs:label")]
        label: Text,

        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
}

impl Definition {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    #[serde(rename = "@context")]
    pub context: HashMap<String, Url>,
    #[serde(rename = "@graph")]
    pub graph: Vec<Definition>,
}

impl Schema {
    pub fn from_reader<R>(rdr: R) -> serde_json::Result<Self>
    where
        R: std::io::Read,
    {
        serde_json::from_reader(rdr)
    }
}
