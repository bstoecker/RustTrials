#[test]
fn test_divide_by_three_fail() {
  if divide_by_three(1) {
    fail!("1 cannot be divided by three.")
  }
}

#[test]
fn test_divide_by_three_ok() {
  if !divide_by_three(3) {
    fail!("3 can be divided by three.")
  }
}

fn divide_by_three(num: int) -> bool {
  if num % 3 == 0 {
    return true
  }
  return false
}
