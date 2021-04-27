use std::str::Chars;
use std::collections::HashMap;

#[allow(dead_code)]
fn dna_strand(dna: &str) -> String {
  // "empty".to_string()
  let char_vec: Chars = dna.chars();
  char_vec.map(|c| match c {
    'A' => "T",
    'T' => "A",
    'G' => "C",
    'C' => "G",
    _ => "",
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
