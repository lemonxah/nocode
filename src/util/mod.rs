use std::time::{SystemTime, UNIX_EPOCH};
use crate::apikey::{ApiKeyError, ApiKey};
use rocket_contrib::json::JsonValue;
use rocket::http::Status;
use rocket::response::status;



pub fn current_millis() -> Result<i64, std::time::SystemTimeError> {
  let now = SystemTime::now();
  let millis = now.duration_since(UNIX_EPOCH)?.as_millis();
  Ok(millis as i64)
}

#[catch(401)]
pub fn unauthorized_catcher(req: &rocket::Request<'_>) -> status::Custom<JsonValue> {
  let o = req.guard::<ApiKey>();
  if o.is_failure() {
    let (_, err) = o.failed().unwrap();
    let (c, msg) = match err {
      ApiKeyError::BadCount => (1, "too many authorization headers"),
      ApiKeyError::Invalid => (2, "invalid token"),
      ApiKeyError::Missing => (3, "token missing"),
      ApiKeyError::Expired => (4, "token expired"),
      ApiKeyError::SysTimeError(_) => (5, "system time error"),
      ApiKeyError::InvalidKeyLength => (6, "invalid secret key length")
    };
    status::Custom(Status::Unauthorized, json!([{"errors": [{"code":c, "message": msg}]}]).into())
  } else {
    status::Custom(Status::Unauthorized, json!([{"errors": [{"code":0, "message": "unknown"}]}]).into())
  }
}

pub fn to_bson_owned<A>(a: &A) -> Result<bson::Document, bson::ser::Error> where A: serde::Serialize {
  let b = bson::to_bson(a)?;
  let doc = b.as_document().unwrap(); // i know i know, but this shouldn't be an issue
  Ok(doc.to_owned())
}

pub fn unauthorized() -> status::Custom<JsonValue> {
  status::Custom(Status::Unauthorized, json!([{"errors": [{"code":5, "message": "does not have access to this call"}]}]).into())
}

#[catch(404)]
pub fn not_found_catcher() -> status::Custom<JsonValue> {
  status::Custom(Status::NotFound, json!([{"errors": [{"code":"1", "message": "not found"}]}]).into())
}

macro_rules! to_vec {
  ($cursor: expr) => {
    {
      let mut v: Vec<_> = vec![];
      for doc in $cursor {
        if let Ok(d) = doc {
          v.push(bson::from_bson(bson::to_bson(&d).unwrap()).unwrap());
        }
      }
      v
    }
  };
}