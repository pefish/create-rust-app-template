pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[cfg(test)]
mod calc_tests {
  use super::*;

  #[test]
  fn add_test() {
    assert_eq!(4, add(2, 2));
  }
}