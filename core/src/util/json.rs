use crate::domain::errors::{CoreError, CoreErrorCode, CoreResult};

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(i64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    pub fn parse(s: &str) -> CoreResult<JsonValue> {
        let mut p = Parser::new(s);
        let v = p.parse_value()?;
        p.skip_ws();
        if !p.eof() {
            return Err(CoreError::new(
                CoreErrorCode::CorruptVault,
                "extra data after JSON",
            ));
        }
        Ok(v)
    }

    pub fn as_object(&self) -> CoreResult<JsonObject<'_>> {
        match self {
            JsonValue::Object(v) => Ok(JsonObject { fields: v }),
            _ => Err(CoreError::new(
                CoreErrorCode::CorruptVault,
                "expected object",
            )),
        }
    }

    pub fn as_array(&self) -> CoreResult<&[JsonValue]> {
        match self {
            JsonValue::Array(v) => Ok(v),
            _ => Err(CoreError::new(
                CoreErrorCode::CorruptVault,
                "expected array",
            )),
        }
    }

    pub fn as_string(&self) -> CoreResult<String> {
        match self {
            JsonValue::String(s) => Ok(s.clone()),
            _ => Err(CoreError::new(
                CoreErrorCode::CorruptVault,
                "expected string",
            )),
        }
    }
}

pub struct JsonObject<'a> {
    fields: &'a [(String, JsonValue)],
}

impl<'a> JsonObject<'a> {
    pub fn get(&self, key: &str) -> Option<&'a JsonValue> {
        self.fields.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn get_string(&self, key: &str) -> CoreResult<String> {
        match self.get(key) {
            Some(JsonValue::String(s)) => Ok(s.clone()),
            _ => Err(CoreError::new(
                CoreErrorCode::CorruptVault,
                format!("expected string field {}", key),
            )),
        }
    }

    pub fn get_i64(&self, key: &str) -> CoreResult<i64> {
        match self.get(key) {
            Some(JsonValue::Number(n)) => Ok(*n),
            _ => Err(CoreError::new(
                CoreErrorCode::CorruptVault,
                format!("expected number field {}", key),
            )),
        }
    }

    pub fn get_array(&self, key: &str) -> CoreResult<&'a [JsonValue]> {
        match self.get(key) {
            Some(JsonValue::Array(v)) => Ok(v),
            _ => Err(CoreError::new(
                CoreErrorCode::CorruptVault,
                format!("expected array field {}", key),
            )),
        }
    }
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            s: input.as_bytes(),
            i: 0,
        }
    }

    fn eof(&self) -> bool {
        self.i >= self.s.len()
    }

    fn peek(&self) -> Option<u8> {
        self.s.get(self.i).copied()
    }

    fn next(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.i += 1;
        Some(b)
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.i += 1;
        }
    }

    fn expect(&mut self, ch: u8) -> CoreResult<()> {
        self.skip_ws();
        match self.next() {
            Some(b) if b == ch => Ok(()),
            _ => Err(CoreError::new(
                CoreErrorCode::CorruptVault,
                format!("expected '{}'", ch as char),
            )),
        }
    }

    fn parse_value(&mut self) -> CoreResult<JsonValue> {
        self.skip_ws();
        match self.peek() {
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b'\"') => Ok(JsonValue::String(self.parse_string()?)),
            Some(b'-' | b'0'..=b'9') => Ok(JsonValue::Number(self.parse_number()?)),
            Some(b't') => {
                self.expect_bytes(b"true")?;
                Ok(JsonValue::Bool(true))
            }
            Some(b'f') => {
                self.expect_bytes(b"false")?;
                Ok(JsonValue::Bool(false))
            }
            Some(b'n') => {
                self.expect_bytes(b"null")?;
                Ok(JsonValue::Null)
            }
            _ => Err(CoreError::new(
                CoreErrorCode::CorruptVault,
                "unexpected token",
            )),
        }
    }

    fn expect_bytes(&mut self, bytes: &[u8]) -> CoreResult<()> {
        for &b in bytes {
            match self.next() {
                Some(x) if x == b => {}
                _ => {
                    return Err(CoreError::new(
                        CoreErrorCode::CorruptVault,
                        "unexpected literal",
                    ))
                }
            }
        }
        Ok(())
    }

    fn parse_object(&mut self) -> CoreResult<JsonValue> {
        self.expect(b'{')?;
        let mut fields = Vec::new();
        self.skip_ws();
        if matches!(self.peek(), Some(b'}')) {
            self.i += 1;
            return Ok(JsonValue::Object(fields));
        }
        loop {
            self.skip_ws();
            let key = self.parse_string()?;
            self.expect(b':')?;
            let val = self.parse_value()?;
            fields.push((key, val));
            self.skip_ws();
            match self.next() {
                Some(b',') => continue,
                Some(b'}') => break,
                _ => {
                    return Err(CoreError::new(
                        CoreErrorCode::CorruptVault,
                        "expected ',' or '}'",
                    ))
                }
            }
        }
        Ok(JsonValue::Object(fields))
    }

    fn parse_array(&mut self) -> CoreResult<JsonValue> {
        self.expect(b'[')?;
        let mut items = Vec::new();
        self.skip_ws();
        if matches!(self.peek(), Some(b']')) {
            self.i += 1;
            return Ok(JsonValue::Array(items));
        }
        loop {
            let v = self.parse_value()?;
            items.push(v);
            self.skip_ws();
            match self.next() {
                Some(b',') => continue,
                Some(b']') => break,
                _ => {
                    return Err(CoreError::new(
                        CoreErrorCode::CorruptVault,
                        "expected ',' or ']'",
                    ))
                }
            }
        }
        Ok(JsonValue::Array(items))
    }

    fn parse_string(&mut self) -> CoreResult<String> {
        self.skip_ws();
        if self.next() != Some(b'\"') {
            return Err(CoreError::new(
                CoreErrorCode::CorruptVault,
                "expected string",
            ));
        }

        let mut out = String::new();
        while let Some(b) = self.next() {
            match b {
                b'\"' => return Ok(out),
                b'\\' => {
                    let esc = self
                        .next()
                        .ok_or_else(|| CoreError::new(CoreErrorCode::CorruptVault, "bad escape"))?;
                    match esc {
                        b'\"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{0008}'),
                        b'f' => out.push('\u{000C}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let code = self.parse_hex4()?;
                            let ch = char::from_u32(code).ok_or_else(|| {
                                CoreError::new(CoreErrorCode::CorruptVault, "bad unicode")
                            })?;
                            out.push(ch);
                        }
                        _ => {
                            return Err(CoreError::new(
                                CoreErrorCode::CorruptVault,
                                "unsupported escape",
                            ))
                        }
                    }
                }
                _ => out.push(b as char),
            }
        }

        Err(CoreError::new(
            CoreErrorCode::CorruptVault,
            "unterminated string",
        ))
    }

    fn parse_hex4(&mut self) -> CoreResult<u32> {
        let mut v: u32 = 0;
        for _ in 0..4 {
            let b = self
                .next()
                .ok_or_else(|| CoreError::new(CoreErrorCode::CorruptVault, "bad unicode"))?;
            v = (v << 4)
                | match b {
                    b'0'..=b'9' => (b - b'0') as u32,
                    b'a'..=b'f' => (b - b'a' + 10) as u32,
                    b'A'..=b'F' => (b - b'A' + 10) as u32,
                    _ => return Err(CoreError::new(CoreErrorCode::CorruptVault, "bad unicode")),
                };
        }
        Ok(v)
    }

    fn parse_number(&mut self) -> CoreResult<i64> {
        self.skip_ws();
        let start = self.i;
        if matches!(self.peek(), Some(b'-')) {
            self.i += 1;
        }
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.i += 1;
        }
        let slice = &self.s[start..self.i];
        let s = std::str::from_utf8(slice)
            .map_err(|_| CoreError::new(CoreErrorCode::CorruptVault, "invalid number"))?;
        s.parse::<i64>()
            .map_err(|_| CoreError::new(CoreErrorCode::CorruptVault, "invalid number"))
    }
}
