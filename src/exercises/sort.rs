// Finish the solution so that it sorts the passed in array of numbers. If the function passes in an empty array or null/nil value then it should return an empty array.

// For example:

// sort_numbers(&vec![1, 2, 3, 10, 5]); // should return vec![1, 2, 3, 5, 10]
// sort_numbers(&vec![]); // should return !vec[]

#[allow(dead_code)]
fn sort_numbers(arr: &Vec<i32>) -> Vec<i32> {
  // unimplemented!();
  let mut v = arr.clone();
  v.sort_unstable();
  v

  // arr.iter().cloned().sorted().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(sort_numbers(&vec![1, 2, 3, 10, 5]), vec![1, 2, 3, 5, 10]);
        assert_eq!(sort_numbers(&vec![]), vec![]);
        assert_eq!(sort_numbers(&vec![20, 2, 10]), vec![2, 10, 20]);
        assert_eq!(sort_numbers(&vec![2, 20, 10]), vec![2, 10, 20]);
        assert_eq!(sort_numbers(&vec![2, 10, 20]), vec![2, 10, 20]);
    }
}
