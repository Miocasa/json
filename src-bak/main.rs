use std::error::Error;
use std::fs;
use jsonparser::Parser;

fn main() -> Result<(), Box<dyn Error>> {


    let str = fs::read_to_string("file.json")?;
    let mut parser = Parser::new(&str);
    let json = parser.parse()?;

    for key in json.keys() {
        println!("Key: {}, value: {:?}", key, json.get(key).unwrap());
    }


    Ok(())
}
