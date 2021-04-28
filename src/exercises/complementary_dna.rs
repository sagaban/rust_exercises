use std::str::Chars;
use std::collections::HashMap;

// Deoxyribonucleic acid (DNA) is a chemical found in the nucleus of cells and carries the "instructions" for the development and functioning of living organisms.

// If you want to know more http://en.wikipedia.org/wiki/DNA

// In DNA strings, symbols "A" and "T" are complements of each other, as "C" and "G". You have function with one side of the DNA (string, except for Haskell); you need to get the other complementary side. DNA strand is never empty or there is no DNA at all (again, except for Haskell).

// More similar exercise are found here http://rosalind.info/problems/list-view/ (source)

// dna_strand("ATTGC") // returns "TAACG"
// dna_strand("GTAT")  // returns "CATA"

#[allow(dead_code)]
fn dna_strand(dna: &str) -> String {
  // "empty".to_string()
  let char_vec: Chars = dna.chars();
  char_vec.map(|c| match c {
    'A' => 'T',
    'T' => 'A',
    'G' => 'C',
    'C' => 'G',
    _ => c,
    // _ => panic!("Unexpected DNA instruction {}", c),

  }).collect()
}

// Other people solution

#[allow(dead_code)]
fn dna_strand_2(dna: &str) -> String {
  let mut result = String::with_capacity(dna.len());
  for c in dna.chars() {
      match c {
          'A' => result.push('T'),
          'T' => result.push('A'),
          'C' => result.push('G'),
          'G' => result.push('C'),
          _ => panic!(),
      }
  }

  result
}

#[allow(dead_code)]
fn dna_strand_3(dna: &str) -> String {
  let complements: HashMap<char, char> = [('A', 'T'), ('T', 'A'), ('C', 'G'), ('G', 'C')].iter().cloned().collect();
  dna.chars().map(|c| complements.get(&c).unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::dna_strand;

    #[test]
    fn returns_expected() {
      assert_eq!(dna_strand("AAAA"),"TTTT");
      assert_eq!(dna_strand("ATTGC"),"TAACG");
      assert_eq!(dna_strand("GTAT"),"CATA");
    }
}
