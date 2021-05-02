// Welcome.

// In this kata you are required to, given a string, replace every letter with its position in the alphabet.

// If anything in the text isn't a letter, ignore it and don't return it.

// "a" = 1, "b" = 2, etc.

// Example
// alphabet_position("The sunset sets at twelve o' clock.")
// Should return "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11" (as a string)

#[allow(dead_code)]
fn alphabet_position(text: &str) -> String {
  String::from(text.chars().map(|c| {
    let code = c.to_ascii_lowercase() as i32 - 96;
    if code > 0 && code < 27 { format!("{} ", code) } else { "".to_string() }
  }).collect::<String>().trim())
}

// Others code
#[allow(dead_code)]
fn alphabet_position_2(text: &str) -> String {
  text.to_lowercase()
      .chars()
      .filter(|c| c >= &'a' && c <= &'z')
      // .filter(|c| c.is_alphabetic())
      .map(|c| (c as u32 - 96).to_string())
      .collect::<Vec<String>>()
      .join(" ")
}

#[test]
fn returns_expected() {
    assert_eq!(
        alphabet_position("The sunset sets at twelve o' clock."),
        "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
    );
    assert_eq!(
        alphabet_position("The narwhal bacons at midnight."),
        "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
    );
    assert_eq!(
        alphabet_position("h^h!iUcW1vX4MXJK&Q6tzTkuJS*tX&"),
        "8 8 9 21 3 23 22 24 13 24 10 11 17 20 26 20 11 21 10 19 20 24".to_string()
    );
    assert_eq!(
        alphabet_position("-=!@#$%^&*()_+[];,./\\{}:|<>?"),
        "".to_string()
    );
}
