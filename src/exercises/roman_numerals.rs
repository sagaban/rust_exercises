// Create a function taking a positive integer as its parameter and returning a string containing the Roman Numeral representation of that integer.

// Modern Roman numerals are written by expressing each digit separately starting with the left most digit and skipping any digit with a value of zero. In Roman numerals 1990 is rendered: 1000=M, 900=CM, 90=XC; resulting in MCMXC. 2008 is written as 2000=MM, 8=VIII; or MMVIII. 1666 uses each Roman symbol in descending order: MDCLXVI.

// Example:

// solution(1000); // should return 'M'
// Help:

// Symbol    Value
// I          1
// V          5
// X          10
// L          50
// C          100
// D          500
// M          1,000

#[allow(dead_code)]
fn num_as_roman(num: i32) -> String {
  if num >= 4000 {
    String::from("Not supported")
  } else {

    let symbols = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
    let mut symbol_index = 0;
    let mut roman = String::new();
    let mut next = num;
    while next != 0 {

      let digit = next % 10;
      for _i in 1..=digit%5%4 {
        roman.push(symbols[symbol_index]);
      }
      if digit > 3 && digit < 9 {
        roman.push(symbols[symbol_index+1]);
      }
      if digit == 9 {
        roman.push(symbols[symbol_index+2]);
      }
      if digit == 4 || digit == 9 {
        roman.push(symbols[symbol_index]);
      }

      symbol_index += 2;
      next /= 10;
    }
    roman.chars().rev().collect()
  }
}

//Others code
#[allow(dead_code)]
fn num_as_roman_2(mut num: i32) -> String {
  let mut letters = String::new();
  let symbols = [(1000, "M"), (900, "CM"),
                 (500,  "D"), (400, "CD"),
                 (100,  "C"), (90,  "XC"),
                 (50,   "L"), (40,  "XL"),
                 (10,   "X"), (9,   "IX"),
                 (5,    "V"), (4,   "IV"),
                 (1,    "I")];
   for &(n, symbol) in symbols.iter() {
     while num >= n {
         letters.push_str(symbol);
         num -= n;
     }
  }
  letters
}

#[test]
fn returns_expected() {
  assert_eq!(num_as_roman(182), "CLXXXII");
  assert_eq!(num_as_roman(1990), "MCMXC");
  assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}
