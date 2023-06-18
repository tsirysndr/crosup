use serde::Serialize;
use serde_json::{json, Value};

use super::matchers::{MatchName, MatchProgram, MatchQueryString, MatchSearch, MatchVersion};

#[derive(Default, Serialize)]
pub struct Query {
    pub max_results: u32,
    pub channel: String,
    pub flakes: bool,
    pub search: Option<MatchSearch>,
    pub program: Option<MatchProgram>,
    pub name: Option<MatchName>,
    pub version: Option<MatchVersion>,
    pub query_string: Option<MatchQueryString>,
}

impl Query {
    pub fn payload(&self) -> Result<Value, serde_json::Error> {
        let mut must: Vec<Value> = vec![json!({
          "match": {
            "type": "package"
          }
        })];

        if let Some(search) = &self.search {
            must.push(json!(search.marshal_json()?));
        }

        if let Some(name) = &self.name {
            must.push(json!(name.marshal_json()?));
        }

        if let Some(program) = &self.program {
            must.push(json!(program.marshal_json()?));
        }

        if let Some(version) = &self.version {
            must.push(json!(version.marshal_json()?));
        }

        if let Some(query_string) = &self.query_string {
            must.push(json!(query_string.marshal_json()?));
        }

        let request_body = json!({
          "from": 0,
          "size": self.max_results,
          "sort": [
            {
              "_score": "desc",
              "package_attr_name": "desc",
              "package_pversion": "desc"
            },
          ],
          "query": {
            "bool": {
              "must": must
            }
          }
        });

        serde_json::to_value(request_body)
    }
}
