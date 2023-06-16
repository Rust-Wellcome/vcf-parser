use std::collections::HashMap;

use crate::headers::Header;
use crate::headers::HeaderValue::{Flat, Nested};

fn is_valid_file_format(input: Header) -> bool {
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_if_key_is_fileformat() {
        let header = Header {key: "fileformat", value: Flat("VCFv4.4")};
        assert_eq!(is_valid_file_format(header), true);
    }

    #[test]
    fn is_invalid_if_key_is_not_fileformat() {
        let header = Header {key: "gileformat", value: Flat("VCFv4.4")};
        assert_eq!(is_valid_file_format(header), false);
    }

    #[test]
    fn is_invalid_if_header_value_nested() {
        let header = Header {key: "fileformat", value: Nested(HashMap::from([("another_key", "VCFv4.4")])) };
        assert_eq!(is_valid_file_format(header), false);
    }
}
