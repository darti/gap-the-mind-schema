use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;
use serde::Deserializer;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::marker::PhantomData;
use url::Url;

const DIGITS: [&str; 10] = [
    "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
];

#[derive(Deserialize, Debug)]
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
#[derive(Deserialize, Debug)]
pub struct Definition {
    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@type")]
    pub ty: OneOrMany<String>,

    #[serde(rename = "rdfs:label")]
    pub label: Text,

    #[serde(rename = "rdfs:subClassOf")]
    pub parents: RefList,

    #[serde(rename = "rdfs:comment")]
    pub comment: Option<Text>,

    #[serde(rename = "schema:domainIncludes")]
    pub domain: RefList,

    #[serde(rename = "schema:rangeIncludes")]
    pub range: RefList,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Definition {}

pub type RefList = Option<OneOrMany<Reference>>;

#[derive(Deserialize, Debug)]
pub struct Reference {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

#[derive(Deserialize, Debug)]
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
            Text::I18n { lang: _, value } => write!(f, "{}", value),
        }
    }
}

impl Text {
    pub fn escape(&self) -> String {
        let s = self.to_string();

        let idx = s.chars().next().and_then(|c| c.to_digit(10));

        if let Some(i) = idx {
            let mut r = DIGITS[i as usize].to_owned();
            r.push_str(&s[1..]);

            return r;
        }

        s
    }
}
