#![feature(proc_macro_hygiene, decl_macro)]
#![feature(option_result_contains)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate bson;
extern crate querylib;
#[macro_use] extern crate serde_json;
extern crate rocket_cors;
extern crate jwt;
extern crate funlib;
extern crate crypto;
extern crate uuid;
extern crate mongodb;
extern crate tokio;
extern crate regex;
extern crate d3ne;
#[macro_use] extern crate anyhow;
extern crate handlebars;
extern crate js_sandbox;


#[macro_use] mod util;
mod nocode;

use rocket::response::NamedFile;
use rocket::http::Status;
use rocket_cors::{CorsOptions, Cors};
use mongodb::{sync::Client, options::ClientOptions};

use std::env;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
enum MainError {
  MongoError(mongodb::error::Error),
  RocketCorsError(rocket_cors::Error),
  SerdeJsonError(serde_json::error::Error),
}

impl From<serde_json::error::Error> for MainError {
  fn from(e: serde_json::error::Error) -> Self {
    MainError::SerdeJsonError(e)
  }
}

impl From<rocket_cors::Error> for MainError {
  fn from(e: rocket_cors::Error) -> Self {
    MainError::RocketCorsError(e)
  }
}

impl From<mongodb::error::Error> for MainError {
  fn from(e: mongodb::error::Error) -> Self {
    MainError::MongoError(e)
  }
}

#[get("/flowtest/healthcheck")]
fn healthcheck() -> Status {
  Status::Ok
}

#[get("/flowview")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("./dist/index.html")
}

#[get("/flowview/edit/<_name>")]
fn edit(_name: String) -> io::Result<NamedFile> {
    NamedFile::open("./dist/index.html")
}

#[get("/flowview/js/<file..>")]
fn js_files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("./dist/js/").join(file)).ok()
}

#[get("/flowview/css/<file..>")]
fn css_files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("./dist/css/").join(file)).ok()
}

#[get("/flowview/img/<file..>")]
fn img_files(file: PathBuf) -> Option<NamedFile> {
  NamedFile::open(Path::new("./dist/img/").join(file)).ok()
}

#[tokio::main]
async fn main() -> std::result::Result<(), MainError> {

  let conn_string = match env::var("DB_CONN_STRING") {
    Ok(cs) => cs,
    Err(_) => "mongodb://localhost:27017".to_owned(),
  };
  
  let mut client_options = ClientOptions::parse(&conn_string)?;

  client_options.app_name = Some("flows".to_string());
  let client = Client::with_options(client_options)?;

  let cors: Cors = match env::var("CORS_JSON") {
    Err(_) => CorsOptions::default(),
    Ok(cors) => serde_json::from_str(&cors)?,
  }.to_cors()?;

  rocket::ignite()
    .mount("/", routes![
      healthcheck,
      nocode::save_flow,
      nocode::get_flows,
      nocode::get_flow,
      nocode::run_flow,
      nocode::test_flow,
      nocode::set_active,
      index,
      edit,
      js_files,
      css_files,
      img_files,
      ])
    .attach(cors)
    .register(catchers![
      util::not_found_catcher,
    ]).manage(client).launch();
    Ok(())
}