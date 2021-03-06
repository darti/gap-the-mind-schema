use crate::elements::Text;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EnumMember {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@type")]
    ty: String,

    #[serde(rename = "rdfs:label")]
    label: Text,

    #[serde(rename = "rdfs:comment")]
    comment: Option<Text>,
}
