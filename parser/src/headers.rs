
//! - [ ] Add implementation
//! - [ ] Check that the first line is fileFormat
//! - [ ] Check that the file format is a valid version
pub fn is_valid(headers: String) -> bool {
  false
}

/// - [ ] Add docs so no need for implementation
#[cfg(test)]
mod tests {

  use super::*;
    // I have maded the assumption that the headers will already be parsed
    // and the double hash ## has been removed
    
    #[test]
    fn test_invalid_without_file_format_on_first_line() {
      let header = String::from("fileDate=20090805");
      assert!(!is_valid(header));
    }

    #[test]
    fn test_invalid_without_valid_file_format_version() {
      let header = String::from("##fileformat=VCFx");
      assert!(!is_valid(header));
    }

}
