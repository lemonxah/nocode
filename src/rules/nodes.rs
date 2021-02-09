use serde_json::Value;
use mongodb::sync::Client;
use std::collections::HashMap;
use std::rc::Rc;
use d3ne::node::*;

fn get_number_field(field: &str, node: &Node, inputs: &InputData) -> f64 {
  let v1 = inputs.get(field).map(|i| i.values().into_iter().next().map(|v| *v.get::<f64>().unwrap()).unwrap());
  v1.or(node.data.get(field).map(|n| n.as_f64().unwrap())).unwrap()
}

fn get_str_field<'a>(field: &str, node: &'a Node, inputs: &'a InputData) -> &'a str {
  let v1 = inputs.get(field).map(|i| i.values().into_iter().next().map(|v| *v.get::<&str>().unwrap()).unwrap());
  v1.or(node.data.get(field).map(|n| n.as_str().unwrap())).unwrap()
}

fn get_json_field<'a>(field: &str, node: &Node, inputs: &InputData) -> Value {
  let v1 = inputs.get(field).map(|i| i.values().into_iter().next().map(|v| *v.get::<&str>().unwrap()).unwrap());
  serde_json::from_str(v1.or(node.data.get(field).map(|n| n.as_str().unwrap())).unwrap()).unwrap()
}

pub fn number(node: Node, _inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  let result = node.data["num"].to_string().parse::<f64>().unwrap();
  map.insert("num".to_string(), IOData {
    data: Box::new(result)
  });
  Rc::new(map)
}

pub fn add(node: Node, inputs: InputData) -> OutputData {
  let num = get_number_field("num", &node, &inputs);
  let num2 = get_number_field("num2", &node, &inputs);

  let mut map = HashMap::new();
  map.insert("num".to_string(), IOData {
    data: Box::new(num + num2)
  });
  Rc::new(map)
}

pub fn multiply(node: Node, inputs: InputData) -> OutputData {
  let num = get_number_field("num", &node, &inputs);
  let num2 = get_number_field("num2", &node, &inputs);

  let mut map = HashMap::new();
  map.insert("num".to_string(), IOData {
    data: Box::new(num * num2)
  });
  Rc::new(map)
}

pub fn mongodb_get(conn: Rc<Client>) -> Box<dyn Fn(Node, InputData) -> OutputData> { 
  Box::new(move |node: Node, inputs: InputData| {

    let dbname = get_str_field("db", &node, &inputs);
    let colname = get_str_field("db", &node, &inputs);

    let db = conn.database(dbname);
    let req = db.collection(colname);
    let mut map = HashMap::new();
    map.insert("json".to_string(), IOData {
      data: json!{{}}
    });
    Rc::new(map)
  })
}
