use crate::class::Class;
use crate::enum_member::EnumMember;
use crate::property::Property;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Definition {
    Property(Property),
    Class(Class),
    EnumMember(EnumMember),
    Other,
}

impl Definition {}
