use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use url::Url;

const DATA_TYPE: &str = "schema:DataType";
const CLASS_TYPE: &str = "rdfs:Class";
const PROPERTY_TYPE: &str = "rdf:Property";

const DATA_TYPES: [&str; 7] = [
    "schema:Float",
    "schema:DateTime",
    "schema:Date",
    "schema:Time",
    "schema:Text",
    "schema:Number",
    "schema:Boolean",
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

impl Definition {
    pub fn parent_types(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        match &self.parents {
            Some(tys) => Box::new(tys.into_iter().map(|t| t.id.as_ref())),
            None => Box::new(std::iter::empty()),
        }
    }

    pub fn domains(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        match &self.domain {
            Some(d) => Box::new(d.into_iter().map(|t| t.id.as_ref())),
            None => Box::new(std::iter::empty()),
        }
    }

    pub fn ranges(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        match &self.range {
            Some(d) => Box::new(d.into_iter().map(|t| t.id.as_ref())),
            None => Box::new(std::iter::empty()),
        }
    }
}

pub enum DefType<'a> {
    Primitive(&'a Definition),
    Property(&'a Definition),
    EnumMember(&'a Definition),
    StructEnum(&'a Definition),
}

impl<'a> From<&'a Definition> for DefType<'a> {
    fn from(d: &'a Definition) -> Self {
        let mut class_type = false;

        if DATA_TYPES.contains(&&d.id.as_ref()) {
            return DefType::Primitive(d);
        }

        for t in d.ty.into_iter() {
            if t == PROPERTY_TYPE {
                return DefType::Property(d);
            } else if t == DATA_TYPE {
                return DefType::Primitive(d);
            } else if t == CLASS_TYPE {
                class_type = true;
            }
        }

        return if class_type {
            DefType::StructEnum(d)
        } else {
            DefType::EnumMember(d)
        };
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
