// Write a program that finds the summation of every number from 1 to num. The number will always be a positive integer greater than 0.

// For example:

// summation(2) -> 3
// 1 + 2

// summation(8) -> 36
// 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8

fn summation(n: i32) -> i32 {
  let mut sum :i32 = 0;
  for i in 1..=n {
      sum += i;
  }
  sum
}

// other people code:
#[allow(dead_code)]
fn summation_2(n: i32) -> i32 {
  (1..=n).sum()
}

#[allow(dead_code)]
fn summation_3(n: i32) -> i32 {
  n * (n + 1) / 2
}

#[allow(dead_code)]
fn summation_4(n: i32) -> i32 {
  match n {
      1 => 1,
      _ => n + summation(n - 1),
  }
}

#[cfg(test)]
mod tests {
    use super::summation;

    #[test]
    fn basic_tests() {
        assert_eq!(summation(1), 1);
        assert_eq!(summation(8), 36);
        assert_eq!(summation(22), 253);
        assert_eq!(summation(100), 5050);
        assert_eq!(summation(213), 22791);
    }
}
