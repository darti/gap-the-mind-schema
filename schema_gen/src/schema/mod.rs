use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::enum_member::EnumMember;
use class::Class;
use property::Property;

pub mod class;
pub mod r#enum;
pub mod enum_member;
pub mod property;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Definition {
    Property(Property),
    Class(Class),
    EnumMember(EnumMember),
    Other,
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
