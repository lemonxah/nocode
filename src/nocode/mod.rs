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
use d3ne::*;

use querylib::{mongo, query, query::*};

// use crate::apikey::{check_access, get_apikey_without_bearer, ApiKey};
use crate::util::current_millis;

#[derive(Debug, Serialize, Clone)]
pub enum NoCodeError {
  MongoError(String),
  SystemTimeError,
  BsonError,
  JsonError,
}

impl std::fmt::Display for NoCodeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      NoCodeError::MongoError(_) => "Database error",
      NoCodeError::SystemTimeError => "SystemTimeError",
      NoCodeError::BsonError => "BsonError",
      NoCodeError::JsonError => "JsonError",
    })
  }
}

impl From<serde_json::error::Error> for NoCodeError {
  fn from(_: serde_json::error::Error) -> Self {
    NoCodeError::JsonError
  }
}

impl From<bson::ser::Error> for NoCodeError {
  fn from(_: bson::ser::Error) -> Self {
    NoCodeError::BsonError
  }
}

impl From<bson::de::Error> for NoCodeError {
  fn from(_: bson::de::Error) -> Self {
    NoCodeError::BsonError
  }
}

impl From<std::time::SystemTimeError> for NoCodeError {
  fn from(_: std::time::SystemTimeError) -> Self {
    NoCodeError::SystemTimeError
  }
}

impl From<mongodb::error::Error> for NoCodeError {
  fn from(e: mongodb::error::Error) -> Self {
    NoCodeError::MongoError(e.to_string())
  }
}

fn setup_engine<'a>(id: &'a str, connection: State<Client>, payload: Value) -> Engine<'a> {
  let mut workers = WorkersBuilder::new();
  let conn: Rc<Client> = Rc::new(connection.clone());
  workers.add(nodes::Input(payload))
    .add(nodes::Output)
    .add(nodes::Number)
    .add(nodes::Float)
    .add(nodes::Text)
    .add(nodes::JsonData)
    .add(nodes::Add)
    .add(nodes::Multiply)
    .add(nodes::ToJson)
    .add(nodes::ToFloat)
    .add(nodes::ToNumber)
    .add(nodes::ToText)
    .add(nodes::Template)
    .add(nodes::HandlebarsWorker)
    .add(nodes::Combine)
    .add(nodes::ScriptWorker)
    .add(nodes::MongodbGet(conn.clone()))
    .add(nodes::MongodbInsert(conn.clone()))
    .add(nodes::MongodbInsert(conn))
    .add(nodes::Head)
    .add(nodes::Nth)
    .add(nodes::ArrayMap)
    .add(nodes::ArrayFlattern)
    .add(nodes::ArraySum)
    .add(nodes::ArrayCount);

  Engine::new(id, workers.build())
}

#[post("/flows/<name>/setactive", format="application/json", data="<data>")]
pub fn set_active(name: String, data: Json<JsonValue>, _cookies: Cookies, conn: State<Client>) -> Result<status::Custom<JsonValue>, NoCodeError> {
  let db = conn.database("flows");
  let metacoll = db.collection("flowsmeta");

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
}

#[post("/flows/<name>?<rev>",format="application/json", data="<data>")]
pub fn run_flow(name: String, rev: Option<i64>, data: Json<JsonValue>, conn: State<Client>/*, apikey: ApiKey */) -> anyhow::Result<status::Custom<JsonValue>> {
  let db = conn.database("flows");
  let coll = db.collection("flows");
  let metacoll = db.collection("flowsmeta");
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
          let engine = setup_engine("flows@1.0.0", conn, data.0.into());
          let nodes = engine.parse_value(entry["flow"].clone()).unwrap();
          let nnodes = nodes.values().cloned().collect::<Vec<_>>().into_iter();
          let start_node = nnodes.clone().filter(|n| n.name == "Input").map(|n| n.id).min().unwrap_or(nnodes.map(|n| n.id).min().unwrap());
      
          match engine.process(&nodes, start_node) {
            Ok(output) => {
              let status = &output["status"].get::<i64>().unwrap();
              let NodeResult(v) = &output["payload"];
              let payload = v.get::<Value>().unwrap();
              Ok(status::Custom(Status::new((**status).try_into().unwrap(), ""), json!({
                "data": payload,
                "rev": getrev,
                "timestamp": current_millis().unwrap()
              }).into()))      
            },
            Err(e) => {
              Ok(status::Custom(Status::new((500).try_into().unwrap(), ""), json!({
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
}

#[get("/flows/<name>?<rev>")]
pub fn get_flow(name: String, rev: Option<i64>, _cookies: Cookies, conn: State<Client>) -> Result<status::Custom<JsonValue>, NoCodeError> {
  let db = conn.database("flows");
  let coll = db.collection("flows");
  let metacoll = db.collection("flowsmeta");
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
}

#[post("/flows",format="application/json", data="<data>")]
pub fn save_flow(data: Json<JsonValue>, _cookies: Cookies, conn: State<Client>) -> Result<status::Custom<JsonValue>, NoCodeError> {
  let name: String = match &data.0["name"] {
    Value::String(n) => n.clone(),
    _ => "_noname".to_string()
  };
  let db = conn.database("flows");
  let coll: Collection = db.collection("flows");
  let metacoll: Collection = db.collection("flowsmeta");
  let pquery = query::parse::from_str(&format!("name == '{}'", name));
  let query = mongo::to_bson(pquery);
  
  let payload_json: Map<String, Value> = serde_json::from_value(data.0["payload"].clone()).unwrap();
  let payload: Document = Document::try_from(payload_json).unwrap();
  let flow_json: Map<String, Value> = serde_json::from_value(data.0["flow"].clone()).unwrap();
  let flow: Document = Document::try_from(flow_json).unwrap();

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
  let res = coll.insert_one(doc!("name": name, "payload": payload, "flow": flow, "rev": nextrev, "timestamp": current_millis().unwrap_or_default()), None);
  match res {
    Ok(_) => {
      Ok(status::Custom(Status::Ok, json!({"rev": nextrev}).into()))
    },
    Err(e) => {
      Ok(status::Custom(Status::InternalServerError, json!({"error": e.to_string()}).into()))
    }
  }
}

#[get("/flows?<limit>&<name>")]
pub fn get_flows(limit: Option<i64>, name: Option<String>, _cookies: Cookies, conn: State<Client>) -> Result<status::Custom<JsonValue>, NoCodeError> {
  let db = conn.database("flows");
  let metacoll = db.collection("flowsmeta");

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
}

#[post("/flowtest",format="application/json", data="<data>")]
pub fn test_flow(data: Json<Value>, _cookies: Cookies, conn: State<Client>) -> Result<status::Custom<JsonValue>, NoCodeError> {
  let payload = data.0["payload"].clone();
  let flow = data.0["flow"].clone();
  let engine = setup_engine("flows@1.0.0", conn, payload);
  let nodes = engine.parse_value(flow).unwrap();
  let nnodes = nodes.values().cloned().collect::<Vec<_>>().into_iter();
  let start_node = nnodes.clone().filter(|n| n.name == "Input").map(|n| n.id).min().unwrap_or(nnodes.map(|n| n.id).min().unwrap());

  match engine.process(&nodes, start_node) {
    Ok(output) => {
      let status = &output["status"].get::<i64>().unwrap();
      let NodeResult(v) = &output["payload"];
      let payload = v.get::<Value>().unwrap();
      Ok(status::Custom(Status::new((**status).try_into().unwrap(), ""), json!({
        "data": payload,
        "rev": -1,
        "timestamp": current_millis().unwrap()
      }).into()))    
    },
    Err(e) => {
      println!("{}", &e);
      Ok(status::Custom(Status::new((200).try_into().unwrap(), ""), json!({
        "error": format!("{:?}", e),
        "rev": -1,
        "timestamp": current_millis().unwrap()
      }).into()))  
    }
  }
}