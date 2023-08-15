use std::collections::HashMap;

use regex::Regex;

use crate::{Header, HeaderValue};

impl<'src> Header<'src> {
    pub fn parse(input: &'src str) -> Result<Self, ParseError> {
        let line = input.trim();
        let (key, value) = line.strip_prefix("##")
            .and_then(|line| line.split_once('='))
            .ok_or(ParseError)?;
        let value = HeaderValue::parse(value)?;
        Ok(Self { key, value })
    }
}

impl<'src> HeaderValue<'src> {
    pub fn parse(input: &'src str) -> Result<Self, ParseError> {
        let re = Regex::new(r#"((?:[^,"]+|(?:"[^"]*"))+)"#).unwrap();
        match input.strip_prefix('<').and_then(|input| input.strip_suffix('>')) {
            None => Ok(Self::Flat(input)),
            Some(pairs) => {
                re.captures_iter(pairs)
                    .map(|c| c.get(0).unwrap().as_str())
                    .map(|pair| pair.split_once('=').ok_or(ParseError))
                    .collect::<Result<HashMap<_, _>, _>>()
                    .map(HeaderValue::Nested)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseError;
