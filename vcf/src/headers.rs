use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'src> {
    pub key: &'src str,
    pub value: HeaderValue<'src>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeaderValue<'src> {
    Flat(&'src str),
    Nested(HashMap<&'src str, &'src str>),
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
                    key: "fileformat",
                    value: HeaderValue::Flat("VCFv1.4"),
                },
                Header {
                    key: "INFO",
                    value: HeaderValue::Nested(HashMap::from([
                        ("abc", "123"),
                        ("xyz", "3125"),
                        ("sfh", "574"),
                    ])),
                },
            ],
        );
    }
}
