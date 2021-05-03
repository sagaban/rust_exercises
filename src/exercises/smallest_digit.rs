// Given a list of digits, return the smallest number that could be formed from these digits, using the digits only once (ignore duplicates).


fn min_value(mut digits: Vec<i32>) -> i32 {
  digits.sort();
  digits.dedup();
  digits.iter().map(|n| n.to_string()).collect::<String>().parse::<i32>().unwrap()
}

// Others code
#[allow(dead_code)]
fn min_value_2(mut digits: Vec<i32>) -> i32 {
  digits.sort();
  digits.dedup();

  digits.into_iter().fold(0, |acc, x| acc * 10 + x)
}

#[test]
fn basic_test() {
  assert_eq!(min_value(vec![1, 3, 1]), 13);
  assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
  assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}
