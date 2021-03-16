use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use url::Url;

const DATA_TYPE: &str = "schema:Datatype";
const CLASS_TYPE: &str = "rdfs:Class";
const PROPERTY_TYPE: &str = "rdf:Property";

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

impl Definition {
    pub fn is_primitive_type(&self) -> bool {
        let types = &self.ty;
        let mut types = types.into_iter();

        types.any(|e| e == DATA_TYPE)
    }

    pub fn is_property(&self) -> bool {
        let types = &self.ty;
        let mut types = types.into_iter();

        types.any(|e| e == PROPERTY_TYPE)
    }

    pub fn is_struct_or_enum(&self) -> bool {
        let mut c = false;
        for t in (&self.ty).into_iter() {
            if t == DATA_TYPE {
                return false;
            } else if t == CLASS_TYPE {
                c = true;
            }
        }

        c
    }

    pub fn is_enum_member(&self) -> bool {
        for t in (&self.ty).into_iter() {
            if t == DATA_TYPE || t == CLASS_TYPE || t == PROPERTY_TYPE {
                return false;
            }
        }

        true
    }
}

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

impl<'a, T> IntoIterator for &'a OneOrMany<T> {
    type Item = <std::slice::Iter<'a, T> as Iterator>::Item;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let items = match self {
            OneOrMany::One(e) => std::slice::from_ref(e),
            OneOrMany::Many(v) => v,
        };

        items.into_iter()
    }
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
