pub mod nodes;

use rocket::http::Cookies;
use serde_json::Map;
use bson::Document;
use mongodb::sync::Collection;
use std::convert::TryFrom;
use mongodb::options::{FindOptions, FindOneAndUpdateOptions};
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

use crate::apikey::{check_access, get_apikey_without_bearer};
use crate::util::current_millis;

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
  workers.put("Float", Box::new(nodes::float));
  workers.put("Text", Box::new(nodes::text));
  workers.put("JSON", Box::new(nodes::json_data));
  workers.put("Add", Box::new(nodes::add));
  workers.put("Multiply", Box::new(nodes::multiply));
  workers.put("ToJSON", Box::new(nodes::to_json));
  workers.put("ToFloat", Box::new(nodes::to_float));
  workers.put("ToNumber", Box::new(nodes::to_number));
  workers.put("ToText", Box::new(nodes::to_text));
  workers.put("Template", Box::new(nodes::template));
  workers.put("Handlebars", Box::new(nodes::handlebars));
  workers.put("Combine", Box::new(nodes::combine));
  workers.put("Script", Box::new(nodes::script));
  workers.put("MongoDB Get", nodes::mongodb_get(Rc::new(conn.clone())));
  workers.put("MongoDB Insert", nodes::mongodb_insert(Rc::new(conn.clone())));
  workers.put("MongoDB Replace", nodes::mongodb_replace(Rc::new(conn.clone())));
  workers.put("Head", Box::new(nodes::head));
  workers.put("Nth", Box::new(nodes::nth));
  workers.put("Array Map", Box::new(nodes::array_map));
  workers.put("Array Flatten", Box::new(nodes::array_flatten));
  workers.put("Array Sum", Box::new(nodes::array_sum));
  workers.put("Array Count", Box::new(nodes::array_count));
  Engine::new(id, workers)
}

#[post("/rules/<name>/setactive", format="application/json", data="<data>")]
pub fn set_active(name: String, data: Json<JsonValue>, cookies: Cookies, conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  let apikey_str = cookies.get("auth").map(|c| c.value().to_string()).or(std::env::var("AUTH").ok()).unwrap_or("".to_string());
  match get_apikey_without_bearer(&apikey_str) {
    Ok(apikey) => {
      if check_access(&apikey, "rules", "save") {
        let db = conn.database("rules");
        let metacoll = db.collection("rulesmeta");

        let options = FindOneAndUpdateOptions::builder()
          .upsert(false)
          .build();
        let pquery = query::parse::from_str(&format!("name == '{}'", name.clone()));
        let query = mongo::to_bson(pquery);
        match data.0["rev"].as_i64() {
          Some(new_active) => {
            let _res = metacoll.find_one_and_update(query.clone(), doc!("$set": doc!("active_rev": new_active)), Some(options))?;
            Ok(status::Custom(Status::Ok, json!({ "name": name, "new_active": new_active }).into()))
          },
          None => Ok(status::Custom(Status::Forbidden, json!({}).into()))
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

#[post("/rules/<name>?<rev>",format="application/json", data="<data>")]
pub fn run_rule(name: String, rev: Option<i64>, data: Json<JsonValue>, cookies: Cookies, conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  let apikey_str = cookies.get("auth").map(|c| c.value().to_string()).or(std::env::var("AUTH").ok()).unwrap_or("".to_string());
  match get_apikey_without_bearer(&apikey_str) {
    Ok(apikey) => {
      if check_access(&apikey, "rules", "run") {
        let db = conn.database("rules");
        let coll = db.collection("rules");
        let metacoll = db.collection("rulesmeta");
        let pquery = query::parse::from_str(&format!("name == '{}'", name));
        let query = mongo::to_bson(pquery.clone());
        let options = FindOptions::builder()
          .limit(1)
          .build();
        let metacursor = metacoll.find(query.clone(), Some(options.clone()))?;
        let meta: Vec<Value> = to_vec!(metacursor);
        
        if meta.len() > 0 {
          let active: i64 = meta[0]["active_rev"].as_i64().unwrap_or(1i64);
          let getrev = rev.unwrap_or(active);
          let q2 = mongo::to_bson(query!(..pquery && "rev" == getrev));
          match coll.find(q2.clone(), Some(options)) {
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
                let status = output["status"].as_ref().unwrap().get::<i64>().unwrap();
                match &output["payload"] {
                  Ok(v) => {
                    let payload = v.get::<Value>().unwrap();
                    Ok(status::Custom(Status::new((*status).try_into().unwrap(), ""), json!({
                      "data": payload,
                      "rev": getrev,
                      "timestamp": current_millis().unwrap()
                    }).into()))    
                  },
                  Err(e) => {
                    Ok(status::Custom(Status::new((*status).try_into().unwrap(), ""), json!({
                      "error": format!("{:?}", e),
                      "rev": getrev,
                      "timestamp": current_millis().unwrap()
                    }).into()))  
                  }
                }
              } else {
                Ok(status::Custom(Status::NotFound, json!({}).into()))
              }
            },
            Err(_) => {
              Ok(status::Custom(Status::InternalServerError, json!({}).into()))
            }
          }
        } else {
          Ok(status::Custom(Status::NotFound, json!({}).into()))
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

#[get("/rules/<name>?<rev>")]
pub fn get_rule(name: String, rev: Option<i64>, cookies: Cookies, conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  let apikey_str = cookies.get("auth").map(|c| c.value().to_string()).or(std::env::var("AUTH").ok()).unwrap_or("".to_string());
  match get_apikey_without_bearer(&apikey_str) {
    Ok(apikey) => {
      if check_access(&apikey, "rules", "read") {
        let db = conn.database("rules");
        let coll = db.collection("rules");
        let metacoll = db.collection("rulesmeta");
        let pquery = query::parse::from_str(&format!("name == '{}'", name));
        let query = mongo::to_bson(pquery.clone());
        let options = FindOptions::builder()
          .limit(1)
          .build();
    
        let metacursor = metacoll.find(query.clone(), Some(options.clone()))?;
        let meta: Vec<Value> = to_vec!(metacursor);
        
        if meta.len() > 0 {
          let latest: i64 = meta[0]["latest_rev"].as_i64().unwrap_or(1i64);
          let getrev: i64 = rev.unwrap_or(latest);
          let q2 = mongo::to_bson(query!(..pquery && "rev" == getrev));
          match coll.find(q2, Some(options)) {
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
          Ok(status::Custom(Status::Ok, json!({}).into()))
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
  let apikey_str = cookies.get("auth").map(|c| c.value().to_string()).or(std::env::var("AUTH").ok()).unwrap_or("".to_string());
  match get_apikey_without_bearer(&apikey_str) {
    Ok(apikey) => {
      if check_access(&apikey, "rules", "save") {
        let name: String = match &data.0["name"] {
          Value::String(n) => n.clone(),
          _ => "_noname".to_string()
        };
        let db = conn.database("rules");
        let coll: Collection = db.collection("rules");
        let metacoll: Collection = db.collection("rulesmeta");
        let pquery = query::parse::from_str(&format!("name == '{}'", name));
        let query = mongo::to_bson(pquery);
        
        let payload_json: Map<String, Value> = serde_json::from_value(data.0["payload"].clone()).unwrap();
        let payload: Document = Document::try_from(payload_json).unwrap();
        let rule_json: Map<String, Value> = serde_json::from_value(data.0["rule"].clone()).unwrap();
        let rule: Document = Document::try_from(rule_json).unwrap();

        let find_options = FindOptions::builder()
          .sort(Some(doc!{"rev": -1}))
          .limit(1)
          .build();

        let meta_options = FindOneAndUpdateOptions::builder()
          .upsert(true)
          .build();

        let old_cursor = coll.find(query.clone(), Some(find_options))?;
        let oldvec: Vec<Document> = to_vec!(old_cursor);
        let last = oldvec.first();
        let lastrev: i64 = last.map(|d| d.get_i64("rev").unwrap_or(0i64)).unwrap_or(0i64);
        let nextrev = lastrev + 1;
        let metaupdate = if nextrev == 1 {
          doc!("$set": doc!("latest_rev": nextrev, "active_rev": nextrev))
        } else {
          doc!("$set": doc!("latest_rev": nextrev))
        };
        let _res = metacoll.find_one_and_update(query.clone(), metaupdate, meta_options);
        let res = coll.insert_one(doc!("name": name, "payload": payload, "rule": rule, "rev": nextrev, "timestamp": current_millis().unwrap_or_default()), None);
        match res {
          Ok(_) => {
            Ok(status::Custom(Status::Ok, json!({"rev": nextrev}).into()))
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

#[get("/rules?<limit>&<name>")]
pub fn get_rules(limit: Option<i64>, name: Option<String>, cookies: Cookies, conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  let apikey_str = cookies.get("auth").map(|c| c.value().to_string()).or(std::env::var("AUTH").ok()).unwrap_or("".to_string());
  match get_apikey_without_bearer(&apikey_str) {
    Ok(apikey) => {
      if check_access(&apikey, "rules", "read") {
        let db = conn.database("rules");
        let metacoll = db.collection("rulesmeta");
    
        let options = FindOptions::builder()
          .limit(limit.unwrap_or(100i64))
          .build();
        let query = name.map(|n| mongo::to_bson(query!("name" == n))).unwrap_or(doc!());
        match metacoll.find(query, Some(options)) {
          Ok(cursor) => {
            let vec: Vec<Value> = to_vec!(cursor);
            let result = serde_json::to_value(&vec).unwrap();
            if vec.len() > 0 {
              Ok(status::Custom(Status::Ok, json!(result).into()))
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

#[post("/ruletest",format="application/json", data="<data>")]
pub fn test_rule(data: Json<Value>, cookies: Cookies, conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  let apikey_str = cookies.get("auth").map(|c| c.value().to_string()).or(std::env::var("AUTH").ok()).unwrap_or("".to_string());
  match get_apikey_without_bearer(&apikey_str) {
    Ok(apikey) => {
      if check_access(&apikey, "rules", "test") {
        let payload = data.0["payload"].clone();
        let rule = data.0["rule"].clone();
        let engine = setup_engine("rules@1.0.0", conn, payload);
        let nodes = engine.parse_value(rule).unwrap();
        let nnodes = nodes.values().cloned().collect::<Vec<_>>().into_iter();
        let start_node = nnodes.clone().filter(|n| n.name == "Input").map(|n| n.id).min().unwrap_or(nnodes.map(|n| n.id).min().unwrap());
        let output = engine.process(&nodes, start_node).unwrap();
        let status = output["status"].as_ref().unwrap().get::<i64>().unwrap();
        match &output["payload"] {
          Ok(v) => {
            let payload = v.get::<Value>().unwrap();
            Ok(status::Custom(Status::new((*status).try_into().unwrap(), ""), json!({
              "data": payload,
              "rev": -1,
              "timestamp": current_millis().unwrap()
            }).into()))    
          },
          Err(e) => {
            Ok(status::Custom(Status::new((*status).try_into().unwrap(), ""), json!({
              "error": format!("{:?}", e),
              "rev": -1,
              "timestamp": current_millis().unwrap()
            }).into()))  
          }
        }
      } else {
        Ok(status::Custom(Status::NotFound, json!({}).into()))
      }
    },
    Err(_) => {
      Ok(status::Custom(Status::Unauthorized, json!({"error": "token missing or invalid"}).into()))
    }
  }
}