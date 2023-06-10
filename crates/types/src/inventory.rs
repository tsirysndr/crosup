use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inventory {
    #[serde(serialize_with = "hcl::ser::labeled_block")]
    pub server: IndexMap<String, ServerConnection>,
}

impl Default for Inventory {
    fn default() -> Self {
        let mut server = IndexMap::new();
        server.insert("server1".into(), ServerConnection::default());
        Self { server }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConnection {
    #[serde(skip_serializing, skip_deserializing)]
    pub name: String,
    pub host: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

impl Default for ServerConnection {
    fn default() -> Self {
        Self {
            name: "server1".to_string(),
            host: "127.0.0.1".to_string(),
            username: "username".to_string(),
            port: Some(22),
        }
    }
}
