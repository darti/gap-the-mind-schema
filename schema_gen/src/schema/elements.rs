use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

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
