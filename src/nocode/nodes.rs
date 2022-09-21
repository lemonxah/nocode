use js_sandbox::AnyError;
use mongodb::options::FindOptions;
use serde_json::Value;
use mongodb::sync::Client;
use std::collections::HashMap;
use std::rc::Rc;
use anyhow::Result;

use regex::Regex;
use js_sandbox::Script;
use handlebars::Handlebars;

use d3ne::*;
use querylib::{mongo, query, query::*};

fn fix_empty_string(s: String) -> Option<String> {
  if s == "".to_string() { None } else { Some(s) }
}

fn fix_empty_str(s: &str) -> Option<&str> {
  if s == "".to_string() { None } else { Some(s) }
}

pub struct Number;
impl Worker for Number {
  fn name(&self) -> &str {
      "Number"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let result = node.get_number_field("num", &input_data)?;
    Ok(OutputDataBuilder::new()
      .data("num", Box::new(result))
      .build())
  }
}
pub struct Float;
impl Worker for Float {
  fn name(&self) -> &str {
      "Float"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let result = node.get_float_number_field("float", &input_data)?;
    Ok(OutputDataBuilder::new()
      .data("float", Box::new(result))
      .build())        
  }
}
pub struct Text;
impl Worker for Text {
  fn name(&self) -> &str {
      "Text"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let result = node.get_string_field("txt", &input_data)?;
    Ok(OutputDataBuilder::new()
      .data("txt", Box::new(result))
      .build())  
  }
}
pub struct Template;
impl Worker for Template {
  fn name(&self) -> &str {
      "Template"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let result = node.get_string_field("template", &input_data)?;  
    Ok(OutputDataBuilder::new()
      .data("template", Box::new(result))
      .build())
  }
}
pub struct Add;
impl Worker for Add {
  fn name(&self) -> &str {
      "Add"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let num = node.get_number_field("num", &input_data)?;
    let num2 = node.get_number_field("num2", &input_data)?;
    Ok(OutputDataBuilder::new()
      .data("num", Box::new(num + num2))
      .build())  
  }
}
pub struct Multiply;
impl Worker for Multiply {
  fn name(&self) -> &str {
      "Multiply"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let num = node.get_number_field("num", &input_data)?;
    let num2 = node.get_number_field("num2", &input_data)?;
    Ok(OutputDataBuilder::new()
      .data("num", Box::new(num * num2))
      .build())        
  }
}
pub struct JsonData;
impl Worker for JsonData {
  fn name(&self) -> &str {
      "Json"
  }
  fn work(&self, node: &Node, _input_data: InputData) -> Result<OutputData> {
    let result = node.data.as_ref().map(|d| d["json"].clone()).unwrap_or_default();
    Ok(OutputDataBuilder::new()
      .data("json", Box::new(result))
      .build())
  }
}
pub struct ToJson;
impl Worker for ToJson {
  fn name(&self) -> &str {
      "ToJson"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let name = node.data.as_ref().and_then(|d| d["name"].as_str()).and_then(fix_empty_str).unwrap_or("data");
    let data = node.get_as_json_field("data", &input_data)?;
    let new_json = match &data {
      Value::String(s) => format!("{{ \"{}\" : \"{}\" }}", name, s),
      _ => format!("{{ \"{}\" : {} }}", name, serde_json::to_string(&data).unwrap())
    };
    let cleaned_json = new_json.replace("\n", "\\n");
    Ok(OutputDataBuilder::new()
      .data("json", Box::new(serde_json::from_str::<Value>(&cleaned_json)?))
      .build())        
  }
}
pub struct ToFloat;
impl Worker for ToFloat {
  fn name(&self) -> &str {
      "ToFloat"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let data = node.get_as_json_field("data", &input_data)?;
    let float: f64 = match &data {
      Value::String(s) => s.parse::<f64>().unwrap_or(0f64),
      Value::Number(n) => n.as_f64().unwrap_or(0f64),
      Value::Bool(b) => if *b { 1f64 } else { 0f64 },
      _ => 0f64,
    };
    Ok(OutputDataBuilder::new()
      .data("float", Box::new(float))
      .build())  
  }
}
pub struct ToNumber;
impl Worker for ToNumber {
  fn name(&self) -> &str {
      "ToNumber"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let data = node.get_as_json_field("data", &input_data)?;
    let num: i64 = match &data {
      Value::String(s) => s.parse::<f64>().unwrap_or(0f64) as i64,
      Value::Number(n) => n.as_f64().unwrap_or(0f64) as i64,
      Value::Bool(b) => if *b { 1i64 } else { 0i64 },
      _ => 0i64,
    };
    Ok(OutputDataBuilder::new()
      .data("num", Box::new(num))
      .build())  
  }
}
pub struct ToText;
impl Worker for ToText {
  fn name(&self) -> &str {
      "ToText"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let data = node.get_as_json_field("data", &input_data)?;
    let text: String = match &data {
      Value::String(s) => s.clone(),
      Value::Number(n) => n.as_f64().unwrap_or(0f64).to_string(),
      Value::Bool(b) => b.to_string(),
      _ => "".to_string(),
    };
    Ok(OutputDataBuilder::new()
      .data("txt", Box::new(text))
      .build())      
  }
}
pub struct ArrayMap;
impl Worker for ArrayMap {
  fn name(&self) -> &str {
      "Array Map"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let (data, fields_str) = array_map_values(node, input_data)?;
    let fields: Vec<&str> = fields_str.split(',').collect();
    let arr: Vec<Value> = serde_json::from_value(data)?;
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
    Ok(OutputDataBuilder::new()
      .data("json", Box::new(res))
      .build())
  }
}
pub struct ArraySum;
impl Worker for ArraySum {
  fn name(&self) -> &str {
      "Array Sum"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let (data, field) = array_sum_values(node, input_data)?;
    let arr: Vec<Value> = serde_json::from_value(data)?;
    let res: Vec<f64> = arr.into_iter().map(|v| v[&field].as_f64().unwrap_or(0f64)).collect();
    let sum: f64 = res.into_iter().sum();
    Ok(OutputDataBuilder::new()
      .data("float", Box::new(sum))
      .build())
  }
}
pub struct ArrayCount;
impl Worker for ArrayCount {
  fn name(&self) -> &str {
      "Array Count"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let data = node.get_json_field("payload", &input_data)?;
    let arr: Vec<Value> = serde_json::from_value(data)?;
    let count = arr.len() as i64;
    Ok(OutputDataBuilder::new()
      .data("num", Box::new(count))
      .build())
  }
}
pub struct ArrayFlattern;
impl Worker for ArrayFlattern {
  fn name(&self) -> &str {
      "Array Flattern"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let data = node.get_json_field("payload", &input_data)?;
    let arr: Vec<Vec<Value>> = serde_json::from_value(data)?;
    let res: Vec<Value> = arr.into_iter().flatten().collect();
    Ok(OutputDataBuilder::new()
      .data("json", Box::new(json!(res)))
      .build())  
  }
}
pub struct Head;
impl Worker for Head {
  fn name(&self) -> &str {
      "Head"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let data = node.get_json_field("payload", &input_data)?;
    Ok(OutputDataBuilder::new()
      .data("json", Box::new(data[0].clone()))
      .build())
  }
}
pub struct Nth;
impl Worker for Nth {
  fn name(&self) -> &str {
      "Nth"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let (data, nth) =  nth_values(node, input_data)?;
    Ok(OutputDataBuilder::new()
      .data("json", Box::new(data[nth as usize].clone()))
      .build())        
  }
}
pub struct Combine;
impl Worker for Combine {
  fn name(&self) -> &str {
      "Combine"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let (data1, data2, data3, data4, name1, name2, name3, name4) = combine_values(node, input_data)?;
    let new_json = format!("{{ \"{}\": {}, \"{}\": {}, \"{}\": {}, \"{}\": {} }}", 
      name1, serde_json::to_string(&data1).unwrap(),
      name2, serde_json::to_string(&data2).unwrap(),
      name3, serde_json::to_string(&data3).unwrap(),
      name4, serde_json::to_string(&data4).unwrap()
    );
    Ok(OutputDataBuilder::new()
      .data("json", Box::new(serde_json::from_str::<Value>(&new_json)?))
      .build())  
  }
}
pub struct HandlebarsWorker;
impl Worker for HandlebarsWorker {
  fn name(&self) -> &str {
      "Handlebars"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let payload = node.get_json_field("payload", &input_data)?;
    let template_txt = node.get_string_field("template", &input_data)?
      .replace("\n", "");
    let reg = Handlebars::new();
    let output = reg.render_template(&template_txt, &json!({"payload": payload}))?;
    let json = serde_json::from_str::<Value>(&output).unwrap_or(json!({}));
    Ok(OutputDataBuilder::new()
      .data("output", Box::new(output))
      .data("json", Box::new(json))
      .build())
  
  }
}
pub struct Input(pub Value);
impl Worker for Input {
  fn name(&self) -> &str {
      "Input"
  }
  fn work(&self, _node: &Node, _input_data: InputData) -> Result<OutputData> {
    Ok(OutputDataBuilder::new()
      .data("payload", Box::new(self.0.clone()))
      .build())
  }
}
pub struct Output;
impl Worker for Output {
  fn name(&self) -> &str {
      "Output"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let status = node.get_number_field("status", &input_data)?;
    let field = "payload";
    let payload = input_data.get(field)
      .and_then(|i| i.get(&node.inputs.clone().map(|i| i[field].connections[0].output.clone()).unwrap_or_default()))
      .map(|data| -> Result<Value> { 
        data.get::<Value>().map(|r| r.clone()).ok_or(anyhow!("Unable to Get Value of Output "))
      }).ok_or(anyhow!("Unable to get the output value from inputs"))??;
    Ok(OutputDataBuilder::new()
      .data("payload", Box::new(payload))
      .data("status", Box::new(status))
      .build())        
  }
}
pub struct ScriptWorker;
impl Worker for ScriptWorker {
  fn name(&self) -> &str {
      "Script"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let mut output_builder = OutputDataBuilder::new();
    match script_values(node, input_data) {
      Ok((_src, payload, name)) => {
        let src = format!("'use strict'; function main(payload) {{ {} }}", _src);
        match Script::from_string(&src) {
          Ok(mut script) => {
            let result: Result<Value, AnyError> = script.call("main", &payload);
            match result {
              Ok(payload) => {
                output_builder.add_data("payload", Box::new(payload));
              },
              Err(e) => {
                let re = Regex::new(r"(?P<error>.+?)\n.+\(sandboxed.js:(?P<line>\d+):(?P<col>\d+)\)\n.+:\d+")?;
                let er = format!("{:?}", &e);
                let rep = format!("$error; @@ {} line: $line, col: $col", name);
                let es = re.replace_all(&er, &rep).to_string();
                bail!(es);
              },
            };
          },
          Err(e) => {
            let re = Regex::new(r"(?P<error>.+?)\n.+sandboxed.js:(?P<line>\d+):(?P<col>\d+)")?;
            let er = format!("{:?}", &e);
            let rep = format!("$error; @@ {} line: $line, col: $col", name);
            let es = re.replace_all(&er, &rep).to_string();
            bail!(es);
          }
        }
      },
      Err(e) => {
        bail!(e);
      },
    };
    Ok(output_builder.build())
  }
}
pub struct MongodbGet(pub Rc<Client>);
impl Worker for MongodbGet {
  fn name(&self) -> &str {
      "MongoDB Get"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let dbname = node.get_string_field("dbname", &input_data).unwrap_or("flows".to_string());
    let colname = node.get_string_field("colname", &input_data).unwrap_or("cache".to_string());
    let squery = node.get_string_field("query", &input_data)?;
    let limit = node.get_number_field("limit", &input_data).unwrap_or(10);

    let db = self.0.database(&dbname);
    let coll = db.collection(&colname);
    let pquery = query::parse::from_str(&squery);
    let query = if dbname == "flows" {
      mongo::to_bson(pquery)
    } else {
      mongo::to_bson(query!(..pquery && "deleted" == false))
    };

    let options = FindOptions::builder()
      .limit(limit)
      .build();
    let mut output_builder = OutputDataBuilder::new();
    match coll.find(query.clone(), Some(options)) {
      Ok(cursor) => {
        let vec: Vec<Value> = to_vec!(cursor);
        let result = serde_json::to_value(vec)?;
        output_builder.add_data("json", Box::new(result));
      },
      Err(e) => {
        bail!(e);
      }
    };
    Ok(output_builder.build())
  }
}
pub struct MongodbInsert(pub Rc<Client>);
impl Worker for MongodbInsert {
  fn name(&self) -> &str {
      "MongoDB Insert"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let dbname = "flows".to_string();
    let colname = node.get_string_field("colname", &input_data)?;
    let mut payload = node.get_json_field("payload", &input_data)?;

    let db = self.0.database(&dbname);
    let coll = db.collection(&colname);
    let mut builder = OutputDataBuilder::new();
    let data = to_bson_owned(&payload)?;
    match coll.insert_one(data, None) {
      Ok(res) => {
        let rr: Value = bson::from_bson(bson::to_bson(&res).unwrap())?;
        payload["_id"] = rr["insertedId"].clone();
        builder.add_data("json", Box::new(payload));
      },
      Err(e) => {
        bail!(e);
      }
    };
    Ok(builder.build())
  }
}
pub struct MongodbReplace(pub Rc<Client>);
impl Worker for MongodbReplace {
  fn name(&self) -> &str {
      "MongoDB Replace"
  }
  fn work(&self, node: &Node, input_data: InputData) -> Result<OutputData> {
    let dbname = "flows".to_string();
    let colname = node.get_string_field("colname", &input_data)?;
    let payload = node.get_json_field("payload", &input_data)?;
    let squery = node.get_string_field("query", &input_data)?;

    let query = mongo::to_bson(query::parse::from_str(&squery));

    let db = self.0.database(&dbname);
    let coll = db.collection(&colname);

    let data = to_bson_owned(&payload)?;
    let mut builder = OutputDataBuilder::new();
    match coll.find_one_and_replace(query, data, None) {
      Ok(Some(res)) => {
        let result: Value = bson::from_bson(bson::to_bson(&res).unwrap())?;
        builder.add_data("json", Box::new(result));
      },
      Ok(None) => {
        bail!("not found");
      },
      Err(e) => {
        bail!(e);
      }
    };
    Ok(builder.build())
  }
}

fn array_map_values(node: &Node, input_data: InputData) -> Result<(Value, String), anyhow::Error> {
  Ok((
    node.get_json_field("payload", &input_data)?,
    node.get_string_field("fields", &input_data)?
  ))
}

fn array_sum_values(node: &Node, input_data: InputData) -> Result<(Value, String), anyhow::Error> {
  Ok((
    node.get_json_field("payload", &input_data)?,
    node.get_string_field("field", &input_data)?
  ))
}

fn nth_values(node: &Node, input_data: InputData) -> Result<(Value, i64), anyhow::Error> {
  Ok((
    node.get_json_field("payload", &input_data)?,
    node.get_number_field("nth", &input_data)?
  ))
}

fn combine_values(node: &Node, input_data: InputData) -> Result<(Value, Value, Value, Value, String, String, String, String), anyhow::Error> {
  Ok((
    node.get_as_json_field_or("data1", &input_data, Some(json!({})))?,
    node.get_as_json_field_or("data2", &input_data, Some(json!({})))?,
    node.get_as_json_field_or("data3", &input_data, Some(json!({})))?,
    node.get_as_json_field_or("data4", &input_data, Some(json!({})))?,
    node.get_string_field("name1", &input_data).ok().and_then(fix_empty_string).unwrap_or("data1".to_string()),
    node.get_string_field("name2", &input_data).ok().and_then(fix_empty_string).unwrap_or("data2".to_string()),
    node.get_string_field("name3", &input_data).ok().and_then(fix_empty_string).unwrap_or("data3".to_string()),
    node.get_string_field("name4", &input_data).ok().and_then(fix_empty_string).unwrap_or("data4".to_string()),
  ))
}

fn script_values(node: &Node, input_data: InputData) -> Result<(String, Value, String), anyhow::Error> {
  Ok((
    node.get_string_field("src", &input_data)?,
    node.get_json_field("payload", &input_data)?,
    node.get_string_field("name", &input_data)?
  ))
}

fn to_bson_owned<A>(a: &A) -> Result<bson::Document> where A: serde::Serialize {
  let b = bson::to_bson(a)?;
  let doc = b.as_document().ok_or(anyhow!("Unable to make document owned"))?;
  Ok(doc.to_owned())
}

#[cfg(test)]
mod node_test {
  use crate::nocode::nodes;
  use d3ne::*;
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
    let mut workers = WorkersBuilder::new();

    workers.add(nodes::Number)
      .add(nodes::JsonData)
      .add(nodes::Combine);

    let engine = Engine::new("tests@1.0.0", workers.build());
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
    let mut workers = WorkersBuilder::new();

    workers.add(nodes::Number);

    let engine = Engine::new("tests@1.0.0", workers.build());
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
    let mut workers = WorkersBuilder::new();

    workers.add(nodes::Number);
    workers.add(nodes::Add);

    let engine = Engine::new("tests@1.0.0", workers.build());
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
    let mut workers = WorkersBuilder::new();

    workers.add(nodes::Number);
    workers.add(nodes::Multiply);

    let engine = Engine::new("tests@1.0.0", workers.build());
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
          "name": "ToJson"
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
          "name": "ToJson"
        }
      },
      "comments": []
    }
    "#;
    let mut workers = WorkersBuilder::new();

    workers.add(nodes::ToJson);

    let engine = Engine::new("tests@1.0.0", workers.build());
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["json"].get::<Value>().unwrap();
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
            "name": "ToJson"
          }
        }
      }
    );
    let mut workers = WorkersBuilder::new();
    workers.add(nodes::ToJson)
      .add(nodes::Template)
      .add(nodes::HandlebarsWorker)
      .add(nodes::Output)
      .add(nodes::Number)
      .add(nodes::Input(json!({"array":["1","2","3","4","5"]})));

    let engine = Engine::new("tests@1.0.0", workers.build());
    let nodes = engine.parse_value(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["payload"].get::<Value>().unwrap();
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
          "name": "ToJson"
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
    let mut workers = WorkersBuilder::new();
    workers.add(nodes::Number)
      .add(nodes::Multiply)
      .add(nodes::ToJson)
      .add(nodes::Template)
      .add(nodes::Output);

    let engine = Engine::new("tests@1.0.0", workers.build());
    let nodes = engine.parse_json(json_data).unwrap();
    let output = engine.process(&nodes, 1);
    let oo = output.unwrap();
    let result = oo["payload"].get::<Value>().unwrap();
    assert_eq!(result, &json!({ "wrapped": 100 }));
  }  
}