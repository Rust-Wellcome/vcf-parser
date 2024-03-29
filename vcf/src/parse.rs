use std::collections::HashMap;

use regex::Regex;
use lazy_static::lazy_static;

use crate::{Header, HeaderValue};

lazy_static! {
    // Repeatedly match either non-comma/non-quote characters or blocks of text enclosed in
    // quotes, until we can't, in which case we're either at a non-quote-enclosed comma or the
    // end of the string.
    static ref HEADER_VALUE_REGEX: Regex = Regex::new(r#"(?:[^,"]+|(?:"[^"]*"))+"#).unwrap();
}

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
        match input.strip_prefix('<').and_then(|input| input.strip_suffix('>')) {
            None => Ok(Self::Flat(input)),
            Some(pairs) => {
                HEADER_VALUE_REGEX.captures_iter(pairs)
                    .map(|c| c.get(0).unwrap().as_str())
                    .map(|pair| pair.split_once('=').ok_or(ParseError))
                    .map(
                        |r| match r {
                            Ok((k, v)) => Ok((k, v.trim_matches('\"'))),
                            x => x,
                        }
                    )
                    .collect::<Result<HashMap<_, _>, _>>()
                    .map(HeaderValue::Nested)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseError;
