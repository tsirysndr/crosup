use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
pub struct MatchSearch {
    pub search: String,
}

impl MatchSearch {
    pub fn marshal_json(&self) -> Result<Value, serde_json::Error> {
        let multi_match_name = format!("multi_match_{}", self.search.replace(" ", "_"));
        let fields = vec![
            "package_attr_name^9",
            "package_attr_name.*^5.3999999999999995",
            "package_programs^9",
            "package_programs.*^5.3999999999999995",
            "package_pname^6",
            "package_pname.*^3.5999999999999996",
            "package_description^1.3",
            "package_description.*^0.78",
            "package_pversion^1.3",
            "package_pversion.*^0.78",
            "package_longDescription^1",
            "package_longDescription.*^0.6",
            "flake_name^0.5",
            "flake_name.*^0.3",
            "flake_resolved.*^99",
        ];

        let mut queries = vec![json!({
            "multi_match": {
                "type": "cross_fields",
                "_name": multi_match_name,
                "query": self.search,
                "fields": fields
            }
        })];

        for term in self.search.split(" ") {
            queries.push(json!({
                "wildcard": {
                    "package_attr_name": {
                        "value": format!("*{}*", term),
                        "case_insensitive": true
                    }
                }
            }));
        }

        let request_body = json!({
            "dis_max": {
                "tie_breaker": 0.7,
                "queries": queries
            }
        });

        serde_json::to_value(request_body)
    }
}

#[derive(Serialize)]
pub struct MatchName {
    pub name: String,
}

impl MatchName {
    pub fn marshal_json(&self) -> Result<Value, serde_json::Error> {
        let queries = vec![
            json!({
                "wildcard": {
                    "package_attr_name": {
                        "value": format!("{}*", self.name)
                    }
                }
            }),
            json!({
                "match": {
                    "package_programs": self.name
                }
            }),
        ];

        let request_body = json!({
            "dis_max": {
                "tie_breaker": 0.7,
                "queries": queries
            }
        });

        serde_json::to_value(request_body)
    }
}

#[derive(Serialize)]
pub struct MatchVersion {
    pub version: String,
}

impl MatchVersion {
    pub fn marshal_json(&self) -> Result<Value, serde_json::Error> {
        let queries = vec![
            json!({
                "wildcard": {
                    "package_pversion": {
                        "value": format!("{}*", self.version)
                    }
                }
            }),
            json!({
                "match": {
                    "package_pversion": self.version
                }
            }),
        ];

        let request_body = json!({
            "dis_max": {
                "tie_breaker": 0.7,
                "queries": queries
            }
        });

        serde_json::to_value(request_body)
    }
}

#[derive(Serialize)]
pub struct MatchProgram {
    pub program: String,
}

impl MatchProgram {
    pub fn marshal_json(&self) -> Result<Value, serde_json::Error> {
        let queries = vec![
            json!({
                "wildcard": {
                    "package_programs": {
                        "value": format!("{}*", self.program)
                    }
                }
            }),
            json!({
                "match": {
                    "package_programs": self.program
                }
            }),
        ];

        let request_body = json!({
            "dis_max": {
                "tie_breaker": 0.7,
                "queries": queries
            }
        });

        serde_json::to_value(request_body)
    }
}

#[derive(Serialize)]
pub struct MatchQueryString {
    pub query_string: String,
}

impl MatchQueryString {
    pub fn marshal_json(&self) -> Result<Value, serde_json::Error> {
        let request_body = json!({
            "query_string": {
                "query": self.query_string
            }
        });

        serde_json::to_value(request_body)
    }
}
