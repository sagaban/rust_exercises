// You are given an array (which will have a length of at least 3, but could be very large) containing integers. The array is either entirely comprised of odd integers or entirely comprised of even integers except for a single integer N. Write a method that takes the array as an argument and returns this "outlier" N.

// Examples
// [2, 4, 0, 100, 4, 11, 2602, 36]
// Should return: 11 (the only odd number)

// [160, 3, 1719, 19, 11, 13, -21]
// Should return: 160 (the only even number)

#[allow(dead_code)]
fn find_outlier(values: &[i32]) -> i32 {
  let r2 = values[1].abs() % 2;
  let r3 = values[2].abs() % 2;
  let mut rest = values[0].abs() % 2;
  if r2 == r3 {
    rest = r2;
  }
  *values.iter().find(|&&v| { v.abs() % 2 != rest}).unwrap()
}


//Others code:
#[allow(dead_code)]
fn find_outlier_2(xs: &[i32]) -> i32 {
  match xs {
      [a, b, c, ..] => {
          let r = if a & 1 == b & 1 { a & 1 } else { c & 1 };
          *xs.iter().find(|&x| x & 1 != r).unwrap()
      }
      _ => unreachable!()
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let t1 = [2,6,8,-10,3];
        // let t2 = [206847684,1056521,7,17,1901,21104421,7,1,35521,1,7781];
        let t2 = [1056521,-7,-17,-1901,21104421,7,1,35521,1,7781, 206847684];
        let t3 = [std::i32::MAX, 0, 1];
        assert_eq!(3, find_outlier(&t1));
        assert_eq!(206847684, find_outlier(&t2));
        assert_eq!(0, find_outlier(&t3));
    }
}
