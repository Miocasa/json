use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub enum JsonValue {
    Null,
    Number(Number),
    String(String),
    Boolean(bool),
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
}

type Res<T> = Result<T, Box<dyn Error>>;
pub struct Parser<'a> {
    bytes: &'a  [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(json: &'a str) -> Self {
        Self {bytes: json.as_bytes(), pos: 0}
    }
    pub fn parse(&mut self) -> Res<HashMap<String, JsonValue>> {

        let map: HashMap<String, JsonValue> = HashMap::new();
        self.skip_ws();



        Ok(map) // todo impl
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        while self.pos < self.bytes.len() && matches!(self.peek(), Some(c) if c.is_ascii_whitespace()) {
            self.pos += 1;
        }
    }

    fn expect(&mut self, b: u8) -> Res<()> {
        self.skip_ws();
        if self.peek() == Some(b) {
            self.pos += 1;
            Ok(())
        } else {
            Err(format!("expected '{}' on position {}", b as char, self.pos).into())
        }
    }

    fn parse_object(&mut self) -> Res<HashMap<String, JsonValue>> {
        self.expect(b'{')?;
        let mut obj: HashMap<String, JsonValue> = HashMap::new();

        self.skip_ws();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            return Ok(obj);
        }

        // todo done impl
        Ok(obj)
    }

    fn parse_string(&mut self) -> Res<String> {
        todo!()
    }
    fn parse_value(&mut self) -> Res<JsonValue> {
        todo!()
    }
    fn parse_number(&mut self) -> Res<JsonValue> {
        todo!()
    }

    fn parse_array(&mut self) -> Res<JsonValue> {
        todo!()
    }


}