use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub key: String,
    pub value: HeaderValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeaderValue {
    Flat(String),
    Nested(HashMap<String, String>),
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{Header, HeaderValue};

    #[test]
    fn test_valid() {
        let input = "\
            ##fileformat=VCFv1.4\n\
            ##INFO=<abc=123,xyz=3125,sfh=574>\n\
        ";
        let headers: Vec<Header> = input
            .lines()
            .map(Header::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(
            headers,
            vec![
                Header {
                    key: "fileformat".to_string(),
                    value: HeaderValue::Flat("VCFv1.4".to_string()),
                },
                Header {
                    key: "INFO".to_string(),
                    value: HeaderValue::Nested(HashMap::from([
                        ("abc".to_string(), "123".to_string()),
                        ("xyz".to_string(), "3125".to_string()),
                        ("sfh".to_string(), "574".to_string()),
                    ])),
                },
            ],
        );
    }

    #[test]
    fn can_parse_when_quoted_text_contains_comma_in_last_key_value_pair() {
        let input = "##FORMAT=<abc=123,xyz=3125,sfh=\"1,574\">";
        let header = Header::parse(input);

        assert_eq!(
            header,
            Ok(
                Header {
                    key: "FORMAT",
                    value: HeaderValue::Nested(HashMap::from([
                        ("abc", "123"),
                        ("xyz", "3125"),
                        ("sfh", "1,574"),
                    ])),
                }
            )
        );
    }

    #[test]
    fn can_parse_when_quoted_text_contains_comma_in_first_key_value_pair() {
        let input = "##FORMAT=<abc=\"1,233\",xyz=3125,sfh=157>";
        let header = Header::parse(input);

        assert_eq!(
            header,
            Ok(
                Header {
                    key: "FORMAT",
                    value: HeaderValue::Nested(HashMap::from([
                        ("abc", "1,233"),
                        ("xyz", "3125"),
                        ("sfh", "157"),
                    ])),
                }
            )
        );
    }
}
