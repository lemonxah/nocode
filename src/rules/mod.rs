pub mod nodes;

use rocket::http::{Cookie, Cookies};
use serde_json::Map;
use bson::Document;
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::sync::Collection;
use std::convert::TryFrom;
use mongodb::options::FindOptions;
use serde_json::Value;
use std::rc::Rc;
use std::convert::TryInto;
use mongodb::sync::Client;
use rocket::State;
use rocket_contrib::json::{JsonValue, Json};
use rocket::response::status;
use rocket::http::Status;
use d3ne::engine::*;
use d3ne::workers::Workers;

use querylib::{mongo, query, query::*};

use crate::apikey::{ApiKey, check_access, get_apikey_without_bearer};

#[derive(Debug, Serialize, Clone)]
pub enum RuleError {
  MongoError(String),
  NoneError,
  SystemTimeError,
  BsonError,
  JsonError,
}

impl std::fmt::Display for RuleError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      RuleError::MongoError(_) => "Database error",
      RuleError::NoneError => "NoneError",
      RuleError::SystemTimeError => "SystemTimeError",
      RuleError::BsonError => "BsonError",
      RuleError::JsonError => "JsonError",
    })
  }
}

impl From<serde_json::error::Error> for RuleError {
  fn from(_: serde_json::error::Error) -> Self {
    RuleError::JsonError
  }
}

impl From<bson::ser::Error> for RuleError {
  fn from(_: bson::ser::Error) -> Self {
    RuleError::BsonError
  }
}

impl From<bson::de::Error> for RuleError {
  fn from(_: bson::de::Error) -> Self {
    RuleError::BsonError
  }
}

impl From<std::time::SystemTimeError> for RuleError {
  fn from(_: std::time::SystemTimeError) -> Self {
    RuleError::SystemTimeError
  }
}

impl From<std::option::NoneError> for RuleError {
  fn from(_: std::option::NoneError) -> Self {
    RuleError::NoneError
  }
}

impl From<mongodb::error::Error> for RuleError {
  fn from(e: mongodb::error::Error) -> Self {
    RuleError::MongoError(e.to_string())
  }
}

fn setup_engine(id: &str, conn: State<Client>, payload: Value) -> Engine {
  let mut workers = Workers::new();
  workers.put("Input", nodes::input(payload));
  workers.put("Output", Box::new(nodes::output));
  workers.put("Number", Box::new(nodes::number));
  workers.put("Text", Box::new(nodes::text));
  workers.put("Add", Box::new(nodes::add));
  workers.put("Multiply", Box::new(nodes::multiply));
  workers.put("Convert", Box::new(nodes::convert));
  workers.put("Template", Box::new(nodes::template));
  workers.put("JsonTemplate", Box::new(nodes::template_json));
  workers.put("Combine", Box::new(nodes::combine));
  workers.put("Script", Box::new(nodes::script));
  workers.put("MongoDB", nodes::mongodb_get(Rc::new(conn.clone())));
  Engine::new(id, workers)
}

#[post("/rules/<name>",format="application/json", data="<data>")]
pub fn run_rule(name: String, data: Json<JsonValue>, cookies: Cookies, conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  let apikey_str = cookies.get("auth").map(|c| c.value()).unwrap_or("");
  match get_apikey_without_bearer(apikey_str) {
    Ok(apikey) => {
      if check_access(&apikey, "rules", "run") {
        let db = conn.database("rules");
        let coll = db.collection("rules");
        let pquery = query::parse::from_str(&format!("name == '{}'", name));
        let query = mongo::to_bson(query!(..pquery && "deleted" == false));
        let options = FindOptions::builder()
          .limit(1)
          .build();
        match coll.find(query.clone(), Some(options)) {
          Ok(cursor) => {
            let vec: Vec<Value> = to_vec!(cursor);
            if vec.len() > 0 {
              let entry = vec[0].clone();
              let engine = setup_engine("rules@1.0.0", conn, data.0.into());
              // let json_data: String = serde_json::to_string(&entry["rule"]).unwrap();
              let nodes = engine.parse_value(entry["rule"].clone()).unwrap();
              let nnodes = nodes.values().cloned().collect::<Vec<_>>().into_iter();
              let start_node = nnodes.clone().filter(|n| n.name == "Input").map(|n| n.id).min().unwrap_or(nnodes.map(|n| n.id).min().unwrap());
          
              let output = engine.process(&nodes, start_node).unwrap();
              let payload = output["payload"].get::<Value>().unwrap();
              let status = output["status"].get::<i64>().unwrap();
              Ok(status::Custom(Status::new((*status).try_into().unwrap(), ""), json!(payload).into()))
            } else {
              Ok(status::Custom(Status::NotFound, json!({}).into()))
            }
          },
          Err(_) => {
            Ok(status::Custom(Status::InternalServerError, json!({}).into()))
          }
        }
      } else {
        Ok(status::Custom(Status::Unauthorized, json!({}).into()))
      }
    },
    Err(_) => {
      Ok(status::Custom(Status::Unauthorized, json!({"error": "token missing or invalid"}).into()))
    }
  }
}

#[get("/rules/<name>")]
pub fn get_rule(name: String, cookies: Cookies, conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  let apikey_str = cookies.get("auth").map(|c| c.value()).unwrap_or("");
  match get_apikey_without_bearer(apikey_str) {
    Ok(apikey) => {
      if check_access(&apikey, "rules", "read") {
        let db = conn.database("rules");
        let coll = db.collection("rules");
        let pquery = query::parse::from_str(&format!("name == '{}'", name));
        let query = mongo::to_bson(query!(..pquery && "deleted" == false));
    
        let options = FindOptions::builder()
          .limit(1)
          .build();
    
        match coll.find(query.clone(), Some(options)) {
          Ok(cursor) => {
            let vec: Vec<Value> = to_vec!(cursor);
            let result = serde_json::to_value(&vec).unwrap();
            if vec.len() > 0 {
              Ok(status::Custom(Status::Ok, json!(result[0]).into()))
            } else {
              Ok(status::Custom(Status::Ok, json!({}).into()))
            }
          },
          Err(_) => {
            Ok(status::Custom(Status::InternalServerError, json!({}).into()))
          }
        }
      } else {
        Ok(status::Custom(Status::Unauthorized, json!({}).into()))
      }
    },
    Err(_) => {
      Ok(status::Custom(Status::Unauthorized, json!({"error": "token missing or invalid"}).into()))
    }
  }
}

#[post("/rules",format="application/json", data="<data>")]
pub fn save_rule(data: Json<JsonValue>, cookies: Cookies, conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  let apikey_str = cookies.get("auth").map(|c| c.value()).unwrap_or("");
  match get_apikey_without_bearer(apikey_str) {
    Ok(apikey) => {
      if check_access(&apikey, "rules", "save") {
        let name: String = match &data.0["name"] {
          Value::String(n) => n.clone(),
          _ => "_noname".to_string()
        };
        let db = conn.database("rules");
        let coll: Collection = db.collection("rules");
        let pquery = query::parse::from_str(&format!("name == '{}'", name));
        let query = mongo::to_bson(query!(..pquery && "deleted" == false));
    
        let options = FindOneAndUpdateOptions::builder()
          .upsert(true)
          .build();
        let payload_json: Map<String, Value> = serde_json::from_value(data.0["payload"].clone()).unwrap();
        let payload: Document = Document::try_from(payload_json).unwrap();
        let rule_json: Map<String, Value> = serde_json::from_value(data.0["rule"].clone()).unwrap();
        let rule: Document = Document::try_from(rule_json).unwrap();
        let update = doc!("$set": doc!("payload": payload, "rule": rule));
        let res = coll.find_one_and_update(query.clone(), update, Some(options));
        match res {
          Ok(v) => {
            Ok(status::Custom(Status::Ok, json!(v).into()))
          },
          Err(e) => {
            Ok(status::Custom(Status::InternalServerError, json!({"error": e.to_string()}).into()))
          }
        }
      } else {
        Ok(status::Custom(Status::Unauthorized, json!({}).into()))
      }
    },
    Err(_) => {
      Ok(status::Custom(Status::Unauthorized, json!({"error": "token missing or invalid"}).into()))
    }
  }
}

#[get("/rules")]
pub fn get_rules(cookies: Cookies, _conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  let apikey_str = cookies.get("auth").map(|c| c.value()).unwrap_or("");
  match get_apikey_without_bearer(apikey_str) {
    Ok(apikey) => {
      if check_access(&apikey, "rules", "read") {
        Ok(status::Custom(Status::Ok, json!({}).into()))
      } else {
        Ok(status::Custom(Status::Unauthorized, json!({}).into()))
      }
    },
    Err(_) => {
      Ok(status::Custom(Status::Unauthorized, json!({"error": "token missing or invalid"}).into()))
    }
  }
}

#[post("/ruletest",format="application/json", data="<data>")]
pub fn test_rule(data: Json<Value>, cookies: Cookies, conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  let apikey_str = cookies.get("auth").map(|c| c.value()).unwrap_or("");
  match get_apikey_without_bearer(apikey_str) {
    Ok(apikey) => {
      if check_access(&apikey, "rules", "test") {
        let engine = setup_engine("rules@1.0.0", conn, data.0["payload"].clone());
        let nodes = engine.parse_value(data.0["rule"].clone()).unwrap();
        let nnodes = nodes.values().cloned().collect::<Vec<_>>().into_iter();
        let start_node = nnodes.clone().filter(|n| n.name == "Input").map(|n| n.id).min().unwrap_or(nnodes.map(|n| n.id).min().unwrap());
    
        let output = engine.process(&nodes, start_node).unwrap();
        let payload = output["payload"].get::<Value>().unwrap();
        let status = output["status"].get::<i64>().unwrap();
        Ok(status::Custom(Status::new((*status).try_into().unwrap(), ""), json!(payload).into()))
      } else {
        Ok(status::Custom(Status::NotFound, json!({}).into()))
      }
    },
    Err(_) => {
      Ok(status::Custom(Status::Unauthorized, json!({"error": "token missing or invalid"}).into()))
    }
  }
}