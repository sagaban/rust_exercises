// This time no story, no theory. The examples below show you how to write function accum:

// Examples:

// accum("abcd") -> "A-Bb-Ccc-Dddd"
// accum("RqaEzty") -> "R-Qq-Aaa-Eeee-Zzzzz-Tttttt-Yyyyyyy"
// accum("cwAt") -> "C-Ww-Aaa-Tttt"
// The parameter of accum is a string which includes only letters from a..z and A..Z.

#[allow(dead_code)]
fn accum(s:&str)->String {
  s.char_indices().map(|(i, c)| {
    let mut acc = c.to_string().to_uppercase();
    //println!("acc: {}",acc);
    for _ in 0..i {
      acc += &c.to_string().to_lowercase();
      //println!("acc: {}",acc);
    }
    acc
  }).collect::<Vec<String>>().join("-")
}

// Others code

#[allow(dead_code)]
fn accum_2(s:&str)->String {
  s.chars().enumerate()
      .map(|(i,c)|c.to_string().to_uppercase() + c.to_string().to_lowercase().repeat(i).as_str())
      .collect::<Vec<String>>()
      .join("-")
}

#[allow(dead_code)]
fn accum_3(s:&str)->String {
  s.chars().enumerate()
  .map(|(i,c)| c.to_string().to_uppercase() +
        &(0..i).map(|_| c.to_string().to_lowercase()).collect::<String>())
  .collect::<Vec<_>>().join("-")
}

#[allow(dead_code)]
fn accum_4(s: &str) -> String {
  let mut vec = Vec::new();
  for (i, c) in s.char_indices() {
      let left =  c.to_string().to_uppercase();
      let right = c.to_string().to_lowercase();
      vec.push(format!("{}{}", left, right.repeat(i)));
  }
  return vec.join("-");
}

#[test]
fn basic_tests() {
  assert_eq!(accum("ZpglnRxqenU"), "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu");
  assert_eq!(accum("NyffsGeyylB"), "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb");
  assert_eq!(accum("MjtkuBovqrU"), "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu");
  assert_eq!(accum("EvidjUnokmM"), "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm");
  assert_eq!(accum("HbideVbxncC"), "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc");
}
