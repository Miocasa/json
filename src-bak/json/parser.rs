use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

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

        let map = self.parse_object()?;
        self.skip_ws();
        if self.pos != self.bytes.len() {
            return Err("trailing characters after JSON".into());
        }
        Ok(map)
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

        loop {
            let key = self.parse_string()?;
            self.skip_ws();
            self.expect(b':')?;
            let value = self.parse_value()?;
            obj.insert(key, value);

            self.skip_ws();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    break;
                }
                _ => return Err(format!("expected ',' or '}}' at position {}", self.pos).into()),
            }

        }

        Ok(obj)
    }

    fn parse_string(&mut self) -> Res<String> {
        self.skip_ws();
        if self.peek() != Some(b'"') {
            return Err(format!("expected string at position {}", self.pos).into());
        }
        self.pos += 1;

        let mut out: Vec<u8> = Vec::new();
        loop {
            match self.peek() {
                None => return Err("unterminated string".into()),
                Some(b'"') => {
                    self.pos += 1;
                    break;
                }
                Some(b'\\') => {
                    self.pos += 1;
                    match self.peek() {
                        Some(b'"') => out.push(b'"'),
                        Some(b'\\') => out.push(b'\\'),
                        Some(b'/') => out.push(b'/'),
                        Some(b'n') => out.push(b'\n'),
                        Some(b't') => out.push(b'\t'),
                        Some(b'r') => out.push(b'\r'),
                        _ => return Err(format!("Unsupported escape at position {}", self.pos).into()),
                    }
                    self.pos += 1;
                }
                Some(c) => {
                    out.push(c);
                    self.pos += 1;
                }
            }
        }
        Ok(String::from_utf8(out)?)
    }
    fn parse_value(&mut self) -> Res<JsonValue> {
        self.skip_ws();
        match self.peek() {
            Some(b'{') => Ok(JsonValue::Object(self.parse_object()?)),
            Some(b'[') => self.parse_array(),
            Some(b'"') => Ok(JsonValue::String(self.parse_string()?)),
            Some(b't') => self.parse_literal(b"true", JsonValue::Boolean(true)),
            Some(b'f') => self.parse_literal(b"false", JsonValue::Boolean(false)),
            Some(b'n') => self.parse_literal(b"null", JsonValue::Null),
            Some(b'-') | Some(b'0'..=b'9') => self.parse_number(),
            _ => Err(format!("unexpected token at position {}", self.pos).into()),
        }

    }

    fn parse_literal(&mut self, lit: &[u8], value: JsonValue) -> Res<JsonValue> {
        if self.bytes[self.pos..].starts_with(lit) {
            self.pos += lit.len();
            Ok(value)
        } else {
            Err(format!("invalid literal at position {}", self.pos).into())
        }
    }
    fn parse_number(&mut self) -> Res<JsonValue> {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        while matches!(self.peek(), Some(b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-')) {
            self.pos += 1;
        }
        let text = std::str::from_utf8(&self.bytes[start..self.pos])?;

        if text.contains(['e', 'E', '.'].as_ref()) {
            Ok(JsonValue::Number(Number::Float(f64::from_str(text)?)))
        } else {
            Ok(JsonValue::Number(Number::Int(i64::from_str(text)?)))
        }



    }

    fn parse_array(&mut self) -> Res<JsonValue> {
        self.expect(b'[')?;
        let mut items = Vec::new();

        self.skip_ws();
        if self.peek() == Some(b']') {
            self.pos += 1;
            return Ok(JsonValue::Array(items));
        }

        loop {
            items.push(self.parse_value()?);

            self.skip_ws();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    break;
                }
                _ => return Err(format!("expected ']' or ',' at position {}", self.pos).into()),
            }
        }

        Ok(JsonValue::Array(items))
    }


}