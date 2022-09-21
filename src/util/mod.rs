use std::time::{SystemTime, UNIX_EPOCH};
use rocket_contrib::json::JsonValue;
use rocket::http::Status;
use rocket::response::status;



pub fn current_millis() -> Result<i64, std::time::SystemTimeError> {
  let now = SystemTime::now();
  let millis = now.duration_since(UNIX_EPOCH)?.as_millis();
  Ok(millis as i64)
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