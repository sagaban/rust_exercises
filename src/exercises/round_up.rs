// Given an integer as input, can you round it to the next (meaning, "higher") multiple of 5?

// Examples:

// input:    output:
// 0    ->   0
// 2    ->   5
// 3    ->   5
// 12   ->   15
// 21   ->   25
// 30   ->   30
// -2   ->   0
// -5   ->   -5
// etc.

#[allow(dead_code)]
fn round_to_next_5(n: i32) -> i32 {
  // unimplemented!()
    let mut result = (n/5) * 5;
//   println!("n: {}", n);
//   println!("r1: {}", result);
    if n%5 != 0 && n > 0 { result += 5; };
//   println!("r2: {}", result);
    result
}


//Others code:
#[allow(dead_code)]
fn round_to_next_5_2 (n: i32) -> i32 {
    n + (5 - n % 5) % 5
}

#[allow(dead_code)]
fn round_to_next_5_3 (n: i32) -> i32 {
    (n as f32 / 5.0).ceil() as i32 * 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(round_to_next_5(1), 5);
    }

    #[test]
    fn test_basic_neg() {
        assert_eq!(round_to_next_5(-1), 0);
    }

    #[test]
    fn test_extended() {
        let tests = [
            [0, 0],
            [1, 5],
            [-1, 0],
            [-5, -5],
            [3, 5],
            [5, 5],
            [7, 10],
            [20, 20],
            [39, 40],
            [990, 990],
            [121, 125],
            [555, 555],
        ];

        for [x, out] in tests.iter() {
            assert_eq!(round_to_next_5(*x), *out);
        }
    }
}
