// ROT13 is a simple letter substitution cipher that replaces a letter with the letter 13 letters after it in the alphabet. ROT13 is an example of the Caesar cipher.

// Create a function that takes a string and returns the string ciphered with Rot13. If there are numbers or special characters included in the string, they should be returned as they are. Only letters from the latin/english alphabet should be shifted, like in the original Rot13 "implementation".


#[allow(dead_code)]
fn rot13(message: &str) -> String {
  message.chars()
  .map(|c| if  c.is_alphabetic() {
    let mut rot = c as u8;
    // 97 a
    // 122 z
    // 65 A
    // 90 Z

    // println!("{}", rot);

    if rot >= 65 && rot<= 90 {
      rot = rot + 13;
      if rot > 90 { rot = rot % 91 + 65;}
    }

    if rot >= 97 && rot <= 122 {
      rot = rot + 13;
      if rot > 122 { rot = rot % 123 + 97;}
    }

    // println!("{}",(c as u8 + 13) % 123);
    // println!("{}",(c as u8 + 13 ) as char);
    rot as char
  } else { c })
  .collect()
}

// Others code
#[allow(dead_code)]
fn rot13_2(message: &str) -> String {
  message.chars().map(|c| {
      match c {
          'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + 13) as char,
          'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - 13) as char,
          _ => c,
      }
  }).collect()
}

#[allow(dead_code)]
fn rot13_3(message: &str) -> String {
  message.chars().map(|c| {
          if c.is_ascii_alphabetic() {
              let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
              (first + (c as u8 + 13 - first) % 26) as char
          } else {
              c
          }
      }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(rot13("azAZ"), "nmNM");
        assert_eq!(rot13("test"), "grfg");
        assert_eq!(rot13("Test"), "Grfg");
    }
}
