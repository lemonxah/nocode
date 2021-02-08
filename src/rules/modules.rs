use std::collections::HashMap;
use std::rc::Rc;
use d3ne::node::*;

pub fn number(node: Node, _inputs: InputData) -> OutputData {
  let mut map = HashMap::new();
  let result = node.data["num"].to_string().parse::<i64>().unwrap();
  map.insert("num".to_string(), IOData {
    data: Box::new(result)
  });
  Rc::new(map)
}

pub fn add(node: Node, inputs: InputData) -> OutputData {
  let inum1 = inputs.get("num").map(|i| i.values().into_iter().next().map(|v| *v.get::<i64>().unwrap()).unwrap());
  let num = inum1.or(node.data.get("num").map(|n| n.as_i64().unwrap())).unwrap();

  let inum2 = inputs.get("num2").map(|i| i.values().into_iter().next().map(|v| *v.get::<i64>().unwrap()).unwrap());
  let num2 = inum2.or(node.data.get("num2").map(|n| n.as_i64().unwrap())).unwrap();

  let mut map = HashMap::new();
  map.insert("num".to_string(), IOData {
    data: Box::new(num + num2)
  });
  Rc::new(map)
}

pub fn multiply(node: Node, inputs: InputData) -> OutputData {
  let inum1 = inputs.get("num").map(|i| i.values().into_iter().next().map(|v| *v.get::<i64>().unwrap()).unwrap());
  let num = inum1.or(node.data.get("num").map(|n| n.as_i64().unwrap())).unwrap();

  let inum2 = inputs.get("num2").map(|i| i.values().into_iter().next().map(|v| *v.get::<i64>().unwrap()).unwrap());
  let num2 = inum2.or(node.data.get("num2").map(|n| n.as_i64().unwrap())).unwrap();

  let mut map = HashMap::new();
  map.insert("num".to_string(), IOData {
    data: Box::new(num * num2)
  });
  Rc::new(map)
}