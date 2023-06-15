use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FleekConfiguration {
    #[serde(serialize_with = "hcl::ser::labeled_block")]
    pub pkg: IndexMap<String, Package>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Package {
    #[serde(skip_serializing, skip_deserializing)]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub postinstall: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_check: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply: Option<bool>,
}
