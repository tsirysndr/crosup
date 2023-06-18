use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub error: Option<Error>,
    pub status: Option<i32>,
    pub hits: Hits,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hits {
    pub hits: Vec<Hit>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseError {
    pub error: Error,
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub r#type: String,
    pub reason: String,
    #[serde(rename = "resource.type")]
    pub resource_type: String,
    #[serde(rename = "resource.id")]
    pub resource_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hit {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_source")]
    pub package: Package,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlakeResolved {
    pub r#type: String,
    pub owner: String,
    pub repo: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub r#type: String,
    #[serde(rename = "package_pname")]
    pub name: String,
    #[serde(rename = "package_attr_name")]
    pub attr_name: String,
    #[serde(rename = "package_attr_set")]
    pub attr_set: String,
    #[serde(rename = "package_outputs")]
    pub outputs: Vec<String>,
    #[serde(
        rename = "package_description",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    #[serde(rename = "package_programs")]
    pub programs: Vec<String>,
    #[serde(rename = "package_homepage")]
    pub homepage: Vec<String>,
    #[serde(rename = "package_pversion")]
    pub version: String,
    #[serde(rename = "package_platforms")]
    pub platforms: Vec<String>,
    #[serde(rename = "package_position")]
    pub position: String,
    #[serde(rename = "package_license")]
    pub licenses: Vec<License>,
    #[serde(rename = "flake_name", skip_serializing_if = "Option::is_none")]
    pub flake_name: Option<String>,
    #[serde(rename = "flake_description", skip_serializing_if = "Option::is_none")]
    pub flake_description: Option<String>,
    #[serde(rename = "flake_resolved", skip_serializing_if = "Option::is_none")]
    pub flake_resolved: Option<FlakeResolved>,
}
