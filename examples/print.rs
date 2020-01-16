use erjson::JSONDocument;

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
        Ok(v) => println!("print: {}", v.to_string()),
        Err(err) => print!("err: {}", err),
    }
}
