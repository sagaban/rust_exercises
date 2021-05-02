// Return the number (count) of vowels in the given string.

// We will consider a, e, i, o, u as vowels for this Kata (but not y).

// The input string will only consist of lower case letters and/or spaces.

#[allow(dead_code)]
fn get_count(string: &str) -> usize {
  let mut vowels_count: usize = 0;
  let nvowels = vec!['a', 'e', 'i', 'o', 'u'];

  for c in string.chars() {
    if nvowels.contains(&c) {
      vowels_count += 1
    }
  }
  vowels_count
}

// Others code

#[allow(dead_code)]
fn get_count_2(string: &str) -> usize {
  string.matches(|x| match x {'a'|'e'|'i'|'o'|'u' => true, _ => false}).count()
}

#[allow(dead_code)]
fn get_count_3(string: &str) -> usize {
  string
      .chars()
      .filter(|&c| "aeiou".contains(c))
      .count()
}

#[allow(dead_code)]
fn get_count_4(s: &str) -> usize {
  s.matches(&['a', 'e', 'i', 'o', 'u'][..]).count()
}

#[test]
fn my_tests() {
  assert_eq!(get_count("abracadabra"), 5);
}
