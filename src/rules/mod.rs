pub mod nodes;

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

use crate::apikey::{ApiKey, check_access};

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
  workers.put("Add", Box::new(nodes::add));
  workers.put("Multiply", Box::new(nodes::multiply));
  workers.put("Convert", Box::new(nodes::convert));
  workers.put("Template", Box::new(nodes::template));
  workers.put("MongoDB", nodes::mongodb_get(Rc::new(conn.clone())));
  Engine::new(id, workers)
}

#[post("/rules/<_name>",format="application/json", data="<_data>")]
pub fn run_rule(_name: String, _data: Json<JsonValue>, apikey: ApiKey, _conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  if check_access(&apikey, "rules", "run") {
    Ok(status::Custom(Status::Ok, json!({}).into()))
  } else {
    Ok(status::Custom(Status::NotFound, json!({}).into()))
  }
}

#[get("/rules/<_name>")]
pub fn get_rule(_name: String, apikey: ApiKey, _conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  if check_access(&apikey, "rules", "read") {
    Ok(status::Custom(Status::Ok, json!({}).into()))
  } else {
    Ok(status::Custom(Status::NotFound, json!({}).into()))
  }
}

#[post("/rules",format="application/json", data="<_data>")]
pub fn save_rule(_data: Json<JsonValue>, apikey: ApiKey, _conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  if check_access(&apikey, "rules", "save") {
    Ok(status::Custom(Status::Ok, json!({}).into()))
  } else {
    Ok(status::Custom(Status::NotFound, json!({}).into()))
  }
}

#[get("/rules")]
pub fn get_rules(apikey: ApiKey, _conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  if check_access(&apikey, "rules", "read") {
    Ok(status::Custom(Status::Ok, json!({}).into()))
  } else {
    Ok(status::Custom(Status::NotFound, json!({}).into()))
  }
}

#[post("/ruletest",format="application/json", data="<data>")]
pub fn test_rule(data: Json<Value>, apikey: ApiKey, conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  if check_access(&apikey, "rules", "test") {
    let engine = setup_engine("rules@1.0.0", conn, data.0["payload"].clone());
    let json_data: String = serde_json::to_string(&data.0["rule"]).unwrap();
    let nodes = engine.parse_json(&json_data).unwrap();
    let output = engine.process(&nodes, 1).unwrap();
    let payload = output["payload"].get::<Value>().unwrap();
    let status = output["status"].get::<i64>().unwrap();
    Ok(status::Custom(Status::new((*status).try_into().unwrap(), ""), json!(payload).into()))
  } else {
    Ok(status::Custom(Status::NotFound, json!({}).into()))
  }
}
