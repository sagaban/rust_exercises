// You get an array of numbers, return the sum of all of the positives ones.

// Example [1,-4,7,12] => 1 + 7 + 12 = 20

// Note: if there is nothing to sum, the sum is default to 0.


#[allow(dead_code)]
fn positive_sum(slice: &[i32]) -> i32 {
  slice.iter().fold(0, |acc, x| if *x > 0 { acc + x } else { acc })
}

// Others code
#[allow(dead_code)]
fn positive_sum_2(arr: &[i32]) -> i32 {
  arr.iter().filter(|x| x.is_positive()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_examples() {
        assert_eq!(positive_sum(&[1,2,3,4,5]), 15);
        assert_eq!(positive_sum(&[1,-2,3,4,5]), 13);
        assert_eq!(positive_sum(&[-1,2,3,4,-5]), 9);
    }

    #[test]
    fn empty_list() {
        assert_eq!(positive_sum(&[]), 0);
    }

    #[test]
    fn all_negative() {
        assert_eq!(positive_sum(&[-1,-2,-3,-4,-5]), 0);
    }
}
