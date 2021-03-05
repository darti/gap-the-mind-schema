use crate::definition::Definition;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

pub mod class;
pub mod definition;
pub mod elements;
pub mod r#enum;
pub mod enum_member;
pub mod property;

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
