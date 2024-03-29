use js_sandbox::AnyError;
use mongodb::options::FindOptions;
use serde_json::Value;
use mongodb::sync::Client;
use std::collections::HashMap;
use std::rc::Rc;

use regex::Regex;
use js_sandbox::Script;
use handlebars::Handlebars;

use d3ne::node::*;
use querylib::{mongo, query, query::*};

fn fix_empty_string(s: String) -> Option<String> {
  if s == "".to_string() { None } else { Some(s) }
}

fn fix_empty_str(s: &str) -> Option<&str> {
  if s == "".to_string() { None } else { Some(s) }
}

pub fn number(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  let result = node.get_number_field("num", &inputs).unwrap();
  map.insert("num".to_string(), iodata!(result));
  Rc::new(map)
}

pub fn float(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  let result = node.get_float_number_field("float", &inputs).unwrap();
  map.insert("float".to_string(), iodata!(result));
  Rc::new(map)
}

pub fn text(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  let result = node.get_string_field("txt", &inputs).unwrap();
  map.insert("txt".to_string(), iodata!(result));
  Rc::new(map)
}

pub fn template(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  let result = node.get_string_field("template", &inputs).unwrap();
  map.insert("template".to_string(), iodata!(result));
  Rc::new(map)
}

pub fn add(node: Node, inputs: InputData) -> OutputData {
  let num = node.get_number_field("num", &inputs).unwrap();
  let num2 = node.get_number_field("num2", &inputs).unwrap();
  let mut map = HashMap::new();
  map.insert("num".to_string(), iodata!(num + num2));
  Rc::new(map)
}

pub fn multiply(node: Node, inputs: InputData) -> OutputData {
  let num = node.get_number_field("num", &inputs).unwrap();
  let num2 = node.get_number_field("num2", &inputs).unwrap();
  let mut map = HashMap::new();
  map.insert("num".to_string(), iodata!(num * num2));
  Rc::new(map)
}

pub fn json_data(node: Node, _inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  let result = node.data["json"].clone();
  map.insert("json".to_string(), iodata!(result));
  Rc::new(map)
}

pub fn to_json(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  let name = node.data["name"].as_str().and_then(fix_empty_str).unwrap_or("data");
  match node.get_as_json_field("data", &inputs) {
    Ok(data) => {
      let new_json = match &data {
        Value::String(s) => format!("{{ \"{}\" : \"{}\" }}", name, s),
        _ => format!("{{ \"{}\" : {} }}", name, serde_json::to_string(&data).unwrap())
      };
      let cleaned_json = new_json.replace("\n", "\\n");
      map.insert("json".to_string(), iodata!(serde_json::from_str::<Value>(&cleaned_json).unwrap()));
    },
    Err(e) => {
      map.insert("json".to_string(), Err(e));
    },
  }
  Rc::new(map)
}

pub fn to_float(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  match node.get_as_json_field("data", &inputs) {
    Ok(data) => {
      let float: f64 = match &data {
        Value::String(s) => s.parse::<f64>().unwrap_or(0f64),
        Value::Number(n) => n.as_f64().unwrap_or(0f64),
        Value::Bool(b) => if *b { 1f64 } else { 0f64 },
        _ => 0f64,
      };
      map.insert("float".to_string(), iodata!(float));
    },
    Err(e) => {
      map.insert("float".to_string(), Err(e));
    }
  }
  Rc::new(map)
}

pub fn to_number(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  match node.get_as_json_field("data", &inputs) {
    Ok(data) => {
      let num: i64 = match &data {
        Value::String(s) => s.parse::<f64>().unwrap_or(0f64) as i64,
        Value::Number(n) => n.as_f64().unwrap_or(0f64) as i64,
        Value::Bool(b) => if *b { 1i64 } else { 0i64 },
        _ => 0i64,
      };
      map.insert("num".to_string(), iodata!(num));
    },
    Err(e) => {
      map.insert("num".to_string(), Err(e));
    }
  }
  Rc::new(map)
}

pub fn to_text(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  match node.get_as_json_field("data", &inputs) {
    Ok(data) => {
      let text: String = match &data {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.as_f64().unwrap_or(0f64).to_string(),
        Value::Bool(b) => b.to_string(),
        _ => "".to_string(),
      };
      map.insert("txt".to_string(), iodata!(text));
    },
    Err(e) => {
      map.insert("txt".to_string(), Err(e));
    }
  }
  Rc::new(map)
}

fn array_map_values(node: Node, inputs: InputData) -> Result<(Value, String), anyhow::Error> {
  Ok((
    node.get_json_field("payload", &inputs)?,
    node.get_string_field("fields", &inputs)?
  ))
}

pub fn array_map(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  match array_map_values(node, inputs) {
    Ok((data, fields_str)) => {
      let fields: Vec<&str> = fields_str.split(',').collect();
      let arr: Vec<Value> = serde_json::from_value(data).unwrap();
      let res: Vec<Value> = arr.into_iter().map(|v| {
        let mut newmap = HashMap::new();
        if fields.len() > 1 {
          for field in &fields {
            let cfield = field.trim();
            newmap.insert(cfield, v[cfield].clone());
          };
          json!(newmap)
        } else if fields.len() == 1 {
          v[&fields[0].trim()].clone()
        } else {
          v
        }
      }).collect();
      map.insert("json".to_string(), iodata!(json!(res)));
    },
    Err(e) => {
      map.insert("json".to_string(), Err(e));
    }
  }
  Rc::new(map)
}

fn array_sum_values(node: Node, inputs: InputData) -> Result<(Value, String), anyhow::Error> {
  Ok((
    node.get_json_field("payload", &inputs)?,
    node.get_string_field("field", &inputs)?
  ))
}

pub fn array_sum(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  match array_sum_values(node, inputs) {
    Ok((data, field)) => {
      let arr: Vec<Value> = serde_json::from_value(data).unwrap();
      let res: Vec<f64> = arr.into_iter().map(|v| v[&field].as_f64().unwrap_or(0f64)).collect();
      let sum: f64 = res.into_iter().sum();
      map.insert("float".to_string(), iodata!(sum));
    },
    Err(e) => {
      map.insert("float".to_string(), Err(e));
    }
  }
  Rc::new(map)
}

pub fn array_count(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  match node.get_json_field("payload", &inputs) {
    Ok(data) => {
      let arr: Vec<Value> = serde_json::from_value(data).unwrap();
      let count = arr.len() as i64;
      map.insert("num".to_string(), iodata!(count));
    },
    Err(e) => {
      map.insert("num".to_string(), Err(e));
    }
  }
  Rc::new(map)
}

pub fn array_flatten(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  match node.get_json_field("payload", &inputs) {
    Ok(data) => {
      let arr: Vec<Vec<Value>> = serde_json::from_value(data).unwrap();
      let res: Vec<Value> = arr.into_iter().flatten().collect();
      map.insert("json".to_string(), iodata!(json!(res)));
    },
    Err(e) => {
      map.insert("json".to_string(), Err(e));
    }
  }
  Rc::new(map)
}

pub fn head(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  match node.get_json_field("payload", &inputs) {
    Ok(data) => {
      map.insert("json".to_string(), iodata!(data[0].clone()));
    },
    Err(e) => {
      map.insert("json".to_string(), Err(e));
    }
  }
  Rc::new(map)
}

fn nth_values(node: Node, inputs: InputData) -> Result<(Value, i64), anyhow::Error> {
  Ok((
    node.get_json_field("payload", &inputs)?,
    node.get_number_field("nth", &inputs)?
  ))
}

pub fn nth(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  match nth_values(node, inputs) {
    Ok((data, _nth)) => {
      map.insert("json".to_string(), iodata!(data[_nth as usize].clone()));
    },
    Err(e) => {
      map.insert("json".to_string(), Err(e));
    }
  }
  
  Rc::new(map)
}

fn combine_values(node: Node, inputs: InputData) -> Result<(Value, Value, Value, Value, String, String, String, String), anyhow::Error> {
  Ok((
    node.get_as_json_field_or("data1", &inputs, Some(json!({})))?,
    node.get_as_json_field_or("data2", &inputs, Some(json!({})))?,
    node.get_as_json_field_or("data3", &inputs, Some(json!({})))?,
    node.get_as_json_field_or("data4", &inputs, Some(json!({})))?,
    node.get_string_field("name1", &inputs).ok().and_then(fix_empty_string).unwrap_or("data1".to_string()),
    node.get_string_field("name2", &inputs).ok().and_then(fix_empty_string).unwrap_or("data2".to_string()),
    node.get_string_field("name3", &inputs).ok().and_then(fix_empty_string).unwrap_or("data3".to_string()),
    node.get_string_field("name4", &inputs).ok().and_then(fix_empty_string).unwrap_or("data4".to_string()),
  ))
}

pub fn combine(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  match combine_values(node, inputs) {
    Ok((data1, data2, data3, data4, name1, name2, name3, name4)) => {
      let new_json = format!("{{ \"{}\": {}, \"{}\": {}, \"{}\": {}, \"{}\": {} }}", 
        name1, serde_json::to_string(&data1).unwrap(),
        name2, serde_json::to_string(&data2).unwrap(),
        name3, serde_json::to_string(&data3).unwrap(),
        name4, serde_json::to_string(&data4).unwrap()
      );
      map.insert("json".to_string(), iodata!(serde_json::from_str::<Value>(&new_json).unwrap()));
    },
    Err(e) => {
      map.insert("json".to_string(), Err(e));
    }
  }

  Rc::new(map)
}

pub fn handlebars(node: Node, inputs: InputData) -> OutputData {
  let payload = node.get_json_field("payload", &inputs).unwrap();
  let template_txt = node.get_string_field("template", &inputs).unwrap_or("".to_string())
    .replace("\n", ""); // remove new lines from templates
  let reg = Handlebars::new();
  let mut map = HashMap::new();
  match reg.render_template(&template_txt, &json!({"payload": payload})) {
    Err(_) => map.insert("err".to_string(), iodata!("")),
    Ok(output) => {
      map.insert("output".to_string(), iodata!(output.clone()));
      map.insert("json".to_string(), iodata!(serde_json::from_str::<Value>(&output).unwrap_or(json!({}))))
    },
  };
  Rc::new(map)
}

pub fn input(payload: Value) -> Box<dyn Fn(Node, InputData) -> OutputData> {
  Box::new(move |_node: Node, _inputs: InputData| {
    let mut map = HashMap::new();
    map.insert("payload".to_string(), iodata!(payload.clone()));
    Rc::new(map)
  })
}

pub fn output(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  let status = node.get_number_field("status", &inputs).unwrap();
  let field = "payload";
  let ress = inputs.get(field)
  .and_then(|i| i.get(&node.inputs[field].connections[0].output))
  .map(|v| match v.as_ref() {
    Ok(rv) => Ok(rv.get::<Value>().map(|r| r.clone()).unwrap()),
    Err(e) => Err(e)
  });
  match ress {
    Some(Ok(payload)) => {
      map.insert("payload".to_string(), iodata!(payload));
    },
    Some(Err(error)) => {
      map.insert("payload".to_string(), Err(anyhow!(format!("{:?}", error))));
    },
    None => {
      map.insert("payload".to_string(), Err(anyhow!("unknown error")));
    },
  }
  map.insert("status".to_string(), iodata!(status));
  Rc::new(map)
}

fn script_values(node: Node, inputs: InputData) -> Result<(String, Value, String), anyhow::Error> {
  Ok((
    node.get_string_field("src", &inputs)?,
    node.get_json_field("payload", &inputs)?,
    node.get_string_field("name", &inputs)?
  ))
}

pub fn script(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  match script_values(node, inputs) {
    Ok((_src, payload, name)) => {
      let src = format!("'use strict'; function main(payload) {{ {} }}", _src);
      match Script::from_string(&src) {
        Ok(mut script) => {
          let result: Result<Value, AnyError> = script.call("main", &payload);
          match result {
            Ok(payload) => {
              map.insert("payload".to_string(), iodata!(payload));
            },
            Err(e) => {
              let re = Regex::new(r"(?P<error>.+?)\n.+\(sandboxed.js:(?P<line>\d+):(?P<col>\d+)\)\n.+:\d+").unwrap();
              let er = format!("{:?}", &e);
              let rep = format!("$error; @@ {} line: $line, col: $col", name);
              let es = re.replace_all(&er, &rep).to_string();
              map.insert("payload".to_string(), Err(anyhow!(es)));
            },
          };
        },
        Err(e) => {
          let re = Regex::new(r"(?P<error>.+?)\n.+sandboxed.js:(?P<line>\d+):(?P<col>\d+)").unwrap();
          let er = format!("{:?}", &e);
          let rep = format!("$error; @@ {} line: $line, col: $col", name);
          let es = re.replace_all(&er, &rep).to_string();
          map.insert("payload".to_string(), Err(anyhow!(es)));
        }
      }
    },
    Err(e) => {
      map.insert("payload".to_string(), Err(e));
    },
  }
  Rc::new(map)
}

pub fn mongodb_get(conn: Rc<Client>) -> Box<dyn Fn(Node, InputData) -> OutputData> { 
  Box::new(move |node: Node, inputs: InputData| {
    let dbname = node.get_string_field("dbname", &inputs).unwrap_or("rules".to_string());
    let colname = node.get_string_field("colname", &inputs).unwrap_or("cache".to_string());
    let squery = node.get_string_field("query", &inputs).unwrap();
    let limit = node.get_number_field("limit", &inputs).unwrap_or(10);

    let db = conn.database(&dbname);
    let coll = db.collection(&colname);
    let pquery = query::parse::from_str(&squery);
    let query = if dbname == "rules" {
      mongo::to_bson(pquery)
    } else {
      mongo::to_bson(query!(..pquery && "deleted" == false))
    };

    let options = FindOptions::builder()
      .limit(limit)
      .build();

    let mut map = HashMap::new();
    match coll.find(query.clone(), Some(options)) {
      Ok(cursor) => {
        let vec: Vec<Value> = to_vec!(cursor);
        let result = serde_json::to_value(vec).unwrap();
        map.insert("json".to_string(), iodata!(result));
      },
      Err(_) => {
        map.insert("json".to_string(), iodata!(json!({"error": "database error"})));
      }
    };
    Rc::new(map)
  })
}

fn to_bson_owned<A>(a: &A) -> bson::Document where A: serde::Serialize {
  let b = bson::to_bson(a).unwrap();
  let doc = b.as_document().unwrap();
  doc.to_owned()
}

pub fn mongodb_insert(conn: Rc<Client>) -> Box<dyn Fn(Node, InputData) -> OutputData> { 
  Box::new(move |node: Node, inputs: InputData| {
    let dbname = "rules".to_string();
    let colname = node.get_string_field("colname", &inputs).unwrap();
    let mut payload = node.get_json_field("payload", &inputs).unwrap();

    let db = conn.database(&dbname);
    let coll = db.collection(&colname);

    let mut map = HashMap::new();
    let data = to_bson_owned(&payload);
    match coll.insert_one(data, None) {
      Ok(res) => {
        let rr: Value = bson::from_bson(bson::to_bson(&res).unwrap()).unwrap();
        payload["_id"] = rr["insertedId"].clone();
        map.insert("json".to_string(), iodata!(payload));
      },
      Err(_) => {
        map.insert("json".to_string(), iodata!(json!({"error": "database error"})));
      }
    };
    Rc::new(map)
  })
}

pub fn mongodb_replace(conn: Rc<Client>) -> Box<dyn Fn(Node, InputData) -> OutputData> { 
  Box::new(move |node: Node, inputs: InputData| {
    let dbname = "rules".to_string();
    let colname = node.get_string_field("colname", &inputs).unwrap();
    let payload = node.get_json_field("payload", &inputs).unwrap();
    let squery = node.get_string_field("query", &inputs).unwrap();

    let query = mongo::to_bson(query::parse::from_str(&squery));

    let db = conn.database(&dbname);
    let coll = db.collection(&colname);

    let mut map = HashMap::new();
    let data = to_bson_owned(&payload);
    match coll.find_one_and_replace(query, data, None) {
      Ok(Some(res)) => {
        let result: Value = bson::from_bson(bson::to_bson(&res).unwrap()).unwrap();
        map.insert("json".to_string(), iodata!(result));
      },
      Ok(None) => {
        map.insert("json".to_string(), iodata!(json!({"error": "notfound"})));
      },
      Err(_) => {
        map.insert("json".to_string(), iodata!(json!({"error": "database error"})));
      }
    };
    Rc::new(map)
  })
}

#[cfg(test)]
mod node_test {
  use crate::rules::nodes;
  use d3ne::{engine::Engine, workers::Workers};
  use serde_json::Value;

  #[test]
  fn combines() {
    let json_data = r#"
    {
      "id": "tests@1.0.0",
      "nodes": {
        "1": {
          "id": 1,
          "name": "Number",
          "data": {
            "num": 25
          },
          "inputs": {},
          "outputs": {
            "num": {
              "connections": [{
                "node": 3,
                "input": "data1",
                "data": {}
              }]
            }
          },
          "position": [-60, 182]
        },
        "2": {
          "id": 2,
          "name": "Json",
          "data": {
            "json": {
              "value1": 1,
              "value2": true,
              "value3": "hello, world"
            }
          },
          "inputs": {},
          "outputs": {
            "json": {
              "connections": [{
                "node": 3,
                "input": "data3",
                "data": {}
              }]
            }
          },
          "position": [-60, 182]
        },
        "3": {
          "id": 3,
          "name": "Combine",
          "data": {
            "name1": "custom1"
          },
          "inputs": {
            "data1": {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            },
            "data3": {
              "connections": [{
                "node": 2,
                "output": "json",
                "data": {}
              }]
            }
          },
          "outputs": {
            "json": {
              "connections": []
            }
          },
          "position": [-60, 182]
        }
      },
      "comments": []
    }
    "#;
    let mut workers = Workers::new();

    workers.put("Number", Box::new(nodes::number));
    workers.put("Json", Box::new(nodes::json_data));
    workers.put("Combine", Box::new(nodes::combine));

    let engine = Engine::new("tests@1.0.0", workers);
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["json"].as_ref().unwrap().get::<Value>().unwrap();
    assert_eq!(result, &json!({ "custom1": 25, "data2": {}, "data3": { "value1": 1, "value2": true, "value3": "hello, world" }, "data4": {} }));
  }

  #[test]
  fn numbers() {
    let json_data = r#"
    {
      "id": "tests@1.0.0",
      "nodes": {
        "1": {
          "id": 1,
          "data": {
            "num": 25
          },
          "inputs": {},
          "outputs": {
            "num": {
              "connections": []
            }
          },
          "position": [-60, 182],
          "name": "Number"
        }
      },
      "comments": []
    }
    "#;
    let mut workers = Workers::new();

    workers.put("Number", Box::new(nodes::number));

    let engine = Engine::new("tests@1.0.0", workers);
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["num"].as_ref().unwrap().get::<i64>().unwrap();
    assert_eq!(result, &25i64);
  }

  #[test]
  fn adds() {
    let json_data = r#"
    {
      "id": "tests@1.0.0",
      "nodes": {
        "1": {
          "id": 1,
          "data": {
            "num": 25
          },
          "inputs": {},
          "outputs": {
            "num": {
              "connections": [{
                "node": 2,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [-60, 182],
          "name": "Number"
        },
        "2": {
          "id": 2,
          "data": {
            "num2": 55
          },
          "inputs": {
            "num" : {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": []
            }
          },
          "position": [-60, 182],
          "name": "Add"
        }
      },
      "comments": []
    }
    "#;
    let mut workers = Workers::new();

    workers.put("Number", Box::new(nodes::number));
    workers.put("Add", Box::new(nodes::add));

    let engine = Engine::new("tests@1.0.0", workers);
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["num"].as_ref().unwrap().get::<i64>().unwrap();
    assert_eq!(result, &80i64);
  }

  #[test]
  fn multiplies() {
    let json_data = r#"
    {
      "id": "tests@1.0.0",
      "nodes": {
        "1": {
          "id": 1,
          "data": {
            "num": 25
          },
          "inputs": {},
          "outputs": {
            "num": {
              "connections": [{
                "node": 2,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [-60, 182],
          "name": "Number"
        },
        "2": {
          "id": 2,
          "data": {
            "num2": 4
          },
          "inputs": {
            "num" : {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": []
            }
          },
          "position": [-60, 182],
          "name": "Multiply"
        }
      },
      "comments": []
    }
    "#;
    let mut workers = Workers::new();

    workers.put("Number", Box::new(nodes::number));
    workers.put("Multiply", Box::new(nodes::multiply));

    let engine = Engine::new("tests@1.0.0", workers);
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["num"].as_ref().unwrap().get::<i64>().unwrap();
    assert_eq!(result, &100i64);
  }

  #[test]
  fn converts() {
    let json_data = r#"
    {
      "id": "tests@1.0.0",
      "nodes": {
        "1": {
          "id": 1,
          "data": {
            "name": "age",
            "data": 25
          },
          "inputs": {},
          "outputs": {
            "json": {
              "connections": [{
                "node": 2,
                "input": "data",
                "data": {}
              }]
            }
          },
          "position": [-60, 182],
          "name": "Convert"
        },
        "2": {
          "id": 2,
          "data": {
            "name": "wrapped",
            "json": 35
          },
          "inputs": {
            "data": {
              "connections": [{
                "node": 1,
                "output": "json",
                "data": {}
              }]
            }
          },
          "outputs": {
            "json": {
              "connections": []
            }
          },
          "position": [-60, 182],
          "name": "Convert"
        }
      },
      "comments": []
    }
    "#;
    let mut workers = Workers::new();

    workers.put("Convert", Box::new(nodes::to_json));

    let engine = Engine::new("tests@1.0.0", workers);
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["json"].as_ref().unwrap().get::<Value>().unwrap();
    assert_eq!(result, &json!({ "wrapped": { "age": 25 } }));
  }

  #[test]
  fn templates() {
    let json_data = json!(
      {
        "id": "tests@1.0.0",
        "nodes": {
          "1": {
            "id": 1,
            "data": {},
            "inputs": {},
            "outputs": {
              "payload": {
                "connections": [{
                  "node": 9,
                  "input": "payload",
                  "data": {}
                }]
              }
            },
            "position": [244.30131347498624, 51.29823997720421],
            "name": "Input"
          },
          "2": {
            "id": 2,
            "data": {},
            "inputs": {
              "payload": {
                "connections": [{
                  "node": 11,
                  "output": "json",
                  "data": {}
                }]
              },
              "status": {
                "connections": [{
                  "node": 3,
                  "output": "num",
                  "data": {}
                }]
              }
            },
            "outputs": {},
            "position": [1207.5925421152297, 100.30867465616544],
            "name": "Output"
          },
          "3": {
            "id": 3,
            "data": {
              "num": 200
            },
            "inputs": {},
            "outputs": {
              "num": {
                "connections": [{
                  "node": 2,
                  "input": "status",
                  "data": {}
                }]
              }
            },
            "position": [954.9352854563016, 267.7901014133855],
            "name": "Number"
          },
          "9": {
            "id": 9,
            "data": {},
            "inputs": {
              "payload": {
                "connections": [{
                  "node": 1,
                  "output": "payload",
                  "data": {}
                }]
              },
              "template": {
                "connections": [{
                  "node": 10,
                  "output": "template",
                  "data": {}
                }]
              }
            },
            "outputs": {
              "output": {
                "connections": [{
                  "node": 11,
                  "input": "data",
                  "data": {}
                }]
              },
              "json": {
                "connections": []
              }
            },
            "position": [653.4320927081462, -16.987709513636673],
            "name": "Handlebars"
          },
          "10": {
            "id": 10,
            "data": {
              "template": "_id in [\n{{#each payload.array}}'{{this}}'{{#unless @last}},{{/unless}}{{/each}}\n]"
            },
            "inputs": {},
            "outputs": {
              "template": {
                "connections": [{
                  "node": 9,
                  "input": "template",
                  "data": {}
                }]
              }
            },
            "position": [8.987686923923889, 212.6419790324861],
            "name": "Template"
          },
          "11": {
            "id": 11,
            "data": {
              "name": "query"
            },
            "inputs": {
              "data": {
                "connections": [{
                  "node": 9,
                  "output": "output",
                  "data": {}
                }]
              }
            },
            "outputs": {
              "json": {
                "connections": [{
                  "node": 2,
                  "input": "payload",
                  "data": {}
                }]
              }
            },
            "position": [908.987619506154, -8.345702171270446],
            "name": "ToJSON"
          }
        }
      }
    );
    let mut workers = Workers::new();

    workers.put("ToJSON", Box::new(nodes::to_json));
    workers.put("Template", Box::new(nodes::template));
    workers.put("Handlebars", Box::new(nodes::handlebars));
    workers.put("Output", Box::new(nodes::output));
    workers.put("Number", Box::new(nodes::number));
    workers.put("Input", nodes::input(json!({"array":["1","2","3","4","5"]})));


    let engine = Engine::new("tests@1.0.0", workers);
    let nodes = engine.parse_value(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["payload"].as_ref().unwrap().get::<Value>().unwrap();
    assert_eq!(result, &json!({"query": r#"_id in ['1','2','3','4','5']"#}));
  }

  #[test]
  fn realtest() {
    let json_data = r#"
    {
      "id": "tests@1.0.0",
      "nodes": {
        "1": {
          "id": 1,
          "data": {
            "num": 25
          },
          "inputs": {},
          "outputs": {
            "num": {
              "connections": [{
                "node": 2,
                "input": "num",
                "data": {}
              }]
            }
          },
          "position": [-60, 182],
          "name": "Number"
        },
        "2": {
          "id": 2,
          "data": {
            "num2": 4
          },
          "inputs": {
            "num" : {
              "connections": [{
                "node": 1,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "num": {
              "connections": [{
                  "node": 3,
                  "input": "data",
                  "data": {}
              }]
            }
          },
          "position": [-60, 182],
          "name": "Multiply"
        },
        "3": {
          "id": 3,
          "data": {
            "name": "wrapped"
          },
          "inputs": {
            "data": {
              "connections": [{
                "node": 2,
                "output": "num",
                "data": {}
              }]
            }
          },
          "outputs": {
            "json": {
              "connections": [{
                  "node": 4,
                  "input": "payload",
                  "data": {}
              }]
            }
          },
          "position": [-60, 182],
          "name": "Convert"
        },
        "4": {
          "id": 4,
          "data": {
            "status": 200
          },
          "inputs": {
            "payload": {
              "connections": [{
                "node": 3,
                "output": "json",
                "data": {}
              }]
            }
          },
          "outputs": {
            "payload": {
              "connections": []
            },
            "status": {
                "connections": []
            }
          },
          "position": [-60, 182],
          "name": "Output"
        }
      },
      "comments": []
    }
    "#;
    let mut workers = Workers::new();

    workers.put("Number", Box::new(nodes::number));
    workers.put("Multiply", Box::new(nodes::multiply));
    workers.put("Convert", Box::new(nodes::to_json));
    workers.put("Template", Box::new(nodes::template));
    workers.put("Output", Box::new(nodes::output));

    let engine = Engine::new("tests@1.0.0", workers);
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["payload"].as_ref().unwrap().get::<Value>().unwrap();
    assert_eq!(result, &json!({ "wrapped": 100 }));
  }  
}