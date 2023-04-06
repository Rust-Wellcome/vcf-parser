use std::collections::HashMap;

pub struct Header<'src> {
    pub key: &'src str,
    pub value: HeaderValue<'src>,
}

pub enum HeaderValue<'src> {
    Flat(&'src str),
    Nested(HashMap<&'src str, &'src str>),
}

/// - [X] Add implementation
/// - [ ] Check that the first line is fileFormat
/// - [ ] Check that the file format is a valid version
pub fn is_valid(headers: &str) -> bool {
    headers.lines()
        .map(Header::parse)
        // Drop the parsed data structures – we're only checking for errors.
        .try_fold((), |_, res| res.map(drop))
        .is_ok()
}

/// - [ ] Add docs so no need for implementation
#[cfg(test)]
mod tests {

    use super::*;

    // I have maded the assumption that the headers will already be parsed
    // and the double hash ## has been removed

    // #[test]
    // fn test_invalid_without_file_format_on_first_line() {
    //     let header = String::from("fileDate=20090805");
    //     assert!(!is_valid(&header));
    // }

    // #[test]
    // fn test_invalid_without_valid_file_format_version() {
    //     let header = String::from("##fileformat=VCFx");
    //     assert!(!is_valid(&header));
    // }

    #[test]
    fn test_valid() {
        let headers = "\
            ##fileformat=VCFv1.4\n\
            ##INFO=<abc=123,xyz=3125,sfh=574>\n\
        ";
        assert!(is_valid(headers))
    }
}
