use erjson::{ JSONDocument, JSONError, JSONValue };

fn main() {
  let data = r#"{
    "name": "John Doe",
    "age": 43,
    "props": { "weight": 76, "height": 2.3 },
    "primes": [ 11, 13, 17, 19, 23 ],
    "colors": [ "red", "blue" ]
  }"#;

  let json = String::from(data);
  let mut doc = JSONDocument::new();
  match doc.parse_string(json) {
    Ok(ref mut v) => {
      println!("name: {}", v.get("name").unwrap()); // John Doe
      println!("age: {}", v.get("age").unwrap()); // 43
      match v {
        JSONValue::Object(hm) => {
          *hm.get_mut("age").unwrap() = JSONValue::Number(45f64);
        }
        _ => {}
      };
      println!("age: {}", v.get("age").unwrap()); // 45
    },
    Err(err) => print!("err: {}", err)
  }

}
