mod nodes;

use std::rc::Rc;
use mongodb::sync::Client;
use mongodb::options::FindOptions;
use rocket::State;
use rocket_contrib::json::{JsonValue, Json};
use rocket::response::status;
use rocket::http::Status;
use funlib::{
  Foldable::*,
  Functor,
};
use d3ne::engine::*;
use d3ne::workers::Workers;
use uuid::Uuid;
use crate::apikey::{ApiKey, check_access};
use crate::util;

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

fn setup_engine(id: &str, conn: State<Client>) -> Engine {
  let mut workers = Workers::new();
  workers.put("Number", Box::new(nodes::number));
  workers.put("Add", Box::new(nodes::add));
  workers.put("Multiply", Box::new(nodes::multiply));
  workers.put("MongoDB", nodes::mongodb_get(Rc::new(conn.clone())));
  Engine::new(id, workers)
}

#[post("/rules/<name>",format="application/json", data="<data>")]
pub fn run_rule(name: String, data: Json<JsonValue>, apikey: ApiKey, conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  if check_access(&apikey, "rules", "run") {
    Ok(status::Custom(Status::Ok, json!({})))
  } else {
    Ok(status::Custom(Status::NotFound, json!({})))
  }
}

#[get("/rules/<name>")]
pub fn get_rule(name: String, apikey: ApiKey, conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  if check_access(&apikey, "rules", "read") {
    Ok(status::Custom(Status::Ok, json!({})))
  } else {
    Ok(status::Custom(Status::NotFound, json!({})))
  }
}

#[post("/rules",format="application/json", data="<data>")]
pub fn save_rule(data: Json<JsonValue>, apikey: ApiKey, conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  if check_access(&apikey, "rules", "save") {
    Ok(status::Custom(Status::Ok, json!({})))
  } else {
    Ok(status::Custom(Status::NotFound, json!({})))
  }
}

#[get("/rules")]
pub fn get_rules(apikey: ApiKey, conn: State<Client>) -> Result<status::Custom<JsonValue>, RuleError> {
  if check_access(&apikey, "rules", "save") {
    Ok(status::Custom(Status::Ok, json!({})))
  } else {
    Ok(status::Custom(Status::NotFound, json!({})))
  }
}

