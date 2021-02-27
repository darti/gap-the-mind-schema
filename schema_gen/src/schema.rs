use crate::class::Class;
use crate::property::Property;
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

impl Text {
    pub fn escape(&self) -> String {
        let s = self.to_string();

        let idx = s.chars().nth(0).and_then(|c| c.to_digit(10));

        if let Some(i) = idx {
            let mut r = DIGITS[i as usize].to_owned();
            r.push_str(&s[1..]);

            return r.to_string();
        }

        s
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
    #[serde(rename = "@id")]
    pub id: String,
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
