use mongodb::options::FindOptions;
use serde_json::Value;
use mongodb::sync::Client;
use std::collections::HashMap;
use std::rc::Rc;

use js_sandbox::Script;
use handlebars::Handlebars;

use d3ne::node::*;
use querylib::{mongo, query, query::*};

pub fn number(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  let result = node.get_number_field("num", &inputs).unwrap();
  map.insert("num".to_string(), iodata!(result));
  Rc::new(map)
}

pub fn text(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  let result = node.get_string_field("txt", &inputs).unwrap();
  map.insert("txt".to_string(), iodata!(result));
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

pub fn convert(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  let data = node.get_as_json_field("data", &inputs).unwrap();
  let new_json = match &data {
    Value::String(s) => format!("{{ \"{}\" : \"{}\" }}", node.data["name"].as_str().unwrap(), s),
    _ => format!("{{ \"{}\" : {} }}", node.data["name"].as_str().unwrap(), serde_json::to_string(&data).unwrap())
  };
  map.insert("json".to_string(), iodata!(serde_json::from_str::<Value>(&new_json).unwrap()));
  Rc::new(map)
}

pub fn combine(node: Node, inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  let data1 = node.get_as_json_field("data1", &inputs).unwrap_or(json!({}));
  let data2 = node.get_as_json_field("data2", &inputs).unwrap_or(json!({}));
  let data3 = node.get_as_json_field("data3", &inputs).unwrap_or(json!({}));
  let data4 = node.get_as_json_field("data4", &inputs).unwrap_or(json!({}));
  let name1 = node.get_string_field("name1", &inputs).unwrap_or("data1".to_string());
  let name2 = node.get_string_field("name2", &inputs).unwrap_or("data2".to_string());
  let name3 = node.get_string_field("name3", &inputs).unwrap_or("data3".to_string());
  let name4 = node.get_string_field("name4", &inputs).unwrap_or("data4".to_string());
  let new_json = format!("{{ \"{}\": {}, \"{}\": {}, \"{}\": {}, \"{}\": {} }}", 
    name1, serde_json::to_string(&data1).unwrap(),
    name2, serde_json::to_string(&data2).unwrap(),
    name3, serde_json::to_string(&data3).unwrap(),
    name4, serde_json::to_string(&data4).unwrap()
  );
  map.insert("json".to_string(), iodata!(serde_json::from_str::<Value>(&new_json).unwrap()));
  Rc::new(map)
}

pub fn template(node: Node, inputs: InputData) -> OutputData {
  let payload = node.get_json_field("payload", &inputs).unwrap();
  let template_txt = node.get_string_field("template", &inputs).unwrap();
  let reg = Handlebars::new();
  let mut map = HashMap::new();
  match reg.render_template(&template_txt, &json!({"payload": payload})) {
    Err(_) => map.insert("err".to_string(), iodata!("")),
    Ok(output) => map.insert("output".to_string(), iodata!(output)),
  };
  Rc::new(map)
}

pub fn template_json(node: Node, inputs: InputData) -> OutputData {
  let payload = node.get_json_field("payload", &inputs).unwrap();
  let template_txt = node.get_string_field("template", &inputs).unwrap();
  let reg = Handlebars::new();
  let mut map = HashMap::new();
  match reg.render_template(&template_txt, &json!({"payload": payload})) {
    Err(_) => map.insert("err".to_string(), iodata!(json!({"error": "unable to render template"}))),
    Ok(output) => map.insert("json".to_string(), iodata!(serde_json::from_str::<Value>(&output).unwrap())),
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
  let result = node.get_json_field("payload", &inputs).unwrap();
  let status = node.get_number_field("status", &inputs).unwrap();
  map.insert("payload".to_string(), iodata!(result));
  map.insert("status".to_string(), iodata!(status));
  Rc::new(map)
}

pub fn script(node: Node, inputs: InputData) -> OutputData {
  let _src = node.get_string_field("src", &inputs).unwrap();
  let payload = node.get_json_field("payload", &inputs).unwrap();
  let src = format!("function main(payload) {{ {} }}", _src);
  let mut script = Script::from_string(&src).expect("js init failed");
  let result: Value = script.call("main", &payload).expect("js call failed");
  let mut map = HashMap::new();
  map.insert("payload".to_string(), iodata!(result));
  Rc::new(map)
}

pub fn mongodb_get(conn: Rc<Client>) -> Box<dyn Fn(Node, InputData) -> OutputData> { 
  Box::new(move |node: Node, inputs: InputData| {
    let dbname = node.get_string_field("dbname", &inputs).unwrap();
    let colname = node.get_string_field("colname", &inputs).unwrap();
    let squery = node.get_string_field("query", &inputs).unwrap();
    let limit = node.get_number_field("limit", &inputs).unwrap_or(10);

    let db = conn.database(&dbname);
    let coll = db.collection(&colname);
    let pquery = query::parse::from_str(&squery);
    let query = mongo::to_bson(query!(..pquery && "deleted" == false));

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
    let result = oo["json"].get::<Value>().unwrap();
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
    let result = oo["num"].get::<i64>().unwrap();
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
    let result = oo["num"].get::<i64>().unwrap();
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
    let result = oo["num"].get::<i64>().unwrap();
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

    workers.put("Convert", Box::new(nodes::convert));

    let engine = Engine::new("tests@1.0.0", workers);
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["json"].get::<Value>().unwrap();
    assert_eq!(result, &json!({ "wrapped": { "age": 25 } }));
  }

  #[test]
  fn templates() {
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
              "connections": [{
                "node": 3,
                "input": "payload",
                "data": {}
              }]
            }
          },
          "position": [-60, 182],
          "name": "Convert"
        },
        "3": {
          "id": 3,
          "data": {
            "template": "{ \"the_age_is\": {{wrapped.age}} }"
          },
          "inputs": {
            "payload": {
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
          "position": [-60, 182],
          "name": "Template"
        }
        
      },
      "comments": []
    }
    "#;
    let mut workers = Workers::new();

    workers.put("Convert", Box::new(nodes::convert));
    workers.put("Template", Box::new(nodes::template));

    let engine = Engine::new("tests@1.0.0", workers);
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["json"].get::<Value>().unwrap();
    assert_eq!(result, &json!({ "the_age_is": 25 }));
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
    workers.put("Convert", Box::new(nodes::convert));
    workers.put("Template", Box::new(nodes::template));
    workers.put("Output", Box::new(nodes::output));

    let engine = Engine::new("tests@1.0.0", workers);
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["payload"].get::<Value>().unwrap();
    assert_eq!(result, &json!({ "wrapped": 100 }));
  }  
}