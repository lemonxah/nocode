use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use funlib::Foldable::*;

use hmac::{Hmac, NewMac};
use jwt::VerifyWithKey;
use sha2::Sha256;

#[derive(Debug, Deserialize)]
pub struct Scope {
  pub scope: String,
  pub actions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Partners {
  pub demand: String,
  pub channel: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiKey {
  pub iss: String,
  pub iat: u64,
  pub jti: String,
  pub sub: String,
  pub legacy: bool,
  pub partners: Option<Partners>,
  pub email: Option<String>,
  pub scopes: Vec<Scope>,
  pub entities: Vec<String>,
  pub exp: Option<u64>,
  pub ests: Option<bool>,
}

#[derive(Debug)]
pub enum ApiKeyError {
  Missing,
  Invalid,
  Expired,
  BadCount,
  InvalidKeyLength,
  SysTimeError(std::time::SystemTimeError),
}

impl From<std::time::SystemTimeError> for ApiKeyError {
  fn from(e: std::time::SystemTimeError) -> Self {
    ApiKeyError::SysTimeError(e)
  }
}

impl From<hmac::crypto_mac::InvalidKeyLength> for ApiKeyError {
  fn from(_: hmac::crypto_mac::InvalidKeyLength) -> Self {
    ApiKeyError::InvalidKeyLength
  }
}

pub fn get_apikey_without_bearer(token: &str) -> Result<ApiKey, ApiKeyError> {
  let skey: Hmac<Sha256> = match env::var("JWT_SECRET") {
    Ok(val) => Hmac::new_varkey(val.as_bytes())?,
    Err(_) => Hmac::new_varkey(b"e72e15b5-04f5-4818-8e8e-88ea69a4817c")?,
  };
  match VerifyWithKey::<ApiKey>::verify_with_key(token, &skey) {
    Ok(claims) => {
      let now = SystemTime::now();
      let in_ms: u128 = now.duration_since(UNIX_EPOCH)?.as_millis();
      if claims.exp == None || claims.exp.any(|&ts| u128::from(ts) > in_ms) {
        Ok(claims)
      } else {
        Err(ApiKeyError::Expired)
      }
    },
    Err(_) => Err(ApiKeyError::Invalid)
  }
}

pub fn get_apikey(token: &str) -> Result<ApiKey, ApiKeyError> {
  if token.starts_with("Bearer ") || token.starts_with("bearer ") {
    let nt = token.replace("Bearer ", "").replace("bearer ", "");
    get_apikey_without_bearer(nt.as_str())
  } else {
    Err(ApiKeyError::Invalid)
  }
}

pub fn check_access(apikey: &ApiKey, scope: &str, action: &str) -> bool {
  apikey.scopes.any(|s| s.scope == "all" && s.actions.contains(&"all".to_owned())) ||
  apikey.scopes.find(|s| s.scope == scope).any(|s| s.actions.contains(&action.to_owned()))
}


impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
  type Error = ApiKeyError;
  fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
    let keys: Vec<_> = request.headers().get("Authorization").collect();
    match keys.len() {
      0 => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
      1 => match get_apikey(keys[0]) {
        Ok(apikey) => Outcome::Success(apikey),
        Err(e) => Outcome::Failure((Status::Unauthorized, e))
      },
      _ => Outcome::Failure((Status::Unauthorized, ApiKeyError::BadCount)),
    }
  }
}