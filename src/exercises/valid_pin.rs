// ATM machines allow 4 or 6 digit PIN codes and PIN codes cannot contain anything but exactly 4 digits or exactly 6 digits.

// If the function is passed a valid PIN string, return true, else return false.

// Examples
// "1234"   -->  true
// "12345"  -->  false
// "a234"   -->  false

#[allow(dead_code)]
fn validate_pin(pin: &str) -> bool {
  (pin.len() == 4 || pin.len() == 6) && !pin.chars().any(|d| {
    println!("d: {}", d);
    println!("is: {}",  d.is_digit(10));
    !d.is_digit(10)
  })
}

// Others code:
#[allow(dead_code)]
fn validate_pin_2(pin: &str) -> bool {
  pin.chars().all(|c| c.is_digit(10)) && (pin.len() == 4 || pin.len() == 6)
}

// #[macro_use] extern crate lazy_static;
// use regex::Regex;

// fn validate_pin(s: &str) -> bool {
//     lazy_static! {
//         static ref RE: Regex = Regex::new(r"\A([0-9]{4}|[0-9]{6})\z").unwrap();
//     }
//     RE.is_match(s)
// }

#[cfg(test)]
mod tests {
    use super::validate_pin;

    #[test]
    fn invalid_length_tests() {
        assert_eq!(validate_pin("1"), false);
        assert_eq!(validate_pin("12"), false);
        assert_eq!(validate_pin("123"), false);
        assert_eq!(validate_pin("12345"), false);
        assert_eq!(validate_pin("1234567"), false);
        assert_eq!(validate_pin("-1234"), false);
        assert_eq!(validate_pin("1.234"), false);
        assert_eq!(validate_pin("-1.234"), false);
        assert_eq!(validate_pin("00000000"), false);
    }

    #[test]
    fn non_digit_chars_tests() {
        assert_eq!(validate_pin("a234"), false);
        assert_eq!(validate_pin(".234"), false);
    }

    #[test]
    fn valid_pin_tests() {
        assert_eq!(validate_pin("1234"), true);
        assert_eq!(validate_pin("0000"), true);
        assert_eq!(validate_pin("1111"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("098765"), true);
        assert_eq!(validate_pin("000000"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("090909"), true);
    }
}
