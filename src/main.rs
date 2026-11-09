use std::error::Error;
use jsonparser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    const STR: &str = r#"{
      "name": "John",
      "age": 30,
      "address": {
        "city": "New York",
        "country": "USA"
      },
      "hobbies": [
        "Reading",
        "Cycling",
        "Photography"
      ]
    }"#;

    let mut json = Parser::new(STR).parse()?;

    for key in json.keys() {
        println!("Key: {}, value: {:?}", key, json.get(key).unwrap());
    }

    Ok(())
}
