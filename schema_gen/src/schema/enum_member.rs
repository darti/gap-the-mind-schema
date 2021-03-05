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

#[cfg(test)]
mod tests {
    use crate::enum_member::EnumMember;

    #[test]
    fn enum_member_deserialize() {
        let doc = r###"
           {
              "@id": "schema:Ear",
              "@type": "schema:PhysicalExam",
              "rdfs:comment": "Ear function assessment with clinical examination.",
              "rdfs:label": "Ear",
              "schema:isPartOf": {
                "@id": "https://health-lifesci.schema.org"
              }
        }
        "###;

        let obj: EnumMember = serde_json::from_str(&doc).unwrap();
    }
}
