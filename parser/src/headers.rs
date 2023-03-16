pub mod headers {
  pub fn it_works() {
    1
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        let result = it_works();
        assert_eq!(result, 1);
    }
}
