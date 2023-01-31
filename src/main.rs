use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader};
use std::env::var;

fn complement(c: char) -> char {
    match c {
        'A' => 'T',
        'C' => 'G',
        'G' => 'C',
        'T' => 'A',
        _ => 'N',
    }
}

fn reverse_complement(s: &str) -> String {
    s.chars().map(|c| complement(c)).rev().collect()
}

fn is_palindrome(s: &str) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }
    let half_len = s.len() / 2_usize;
    s[0..half_len] == reverse_complement(&s[half_len..])
}

fn load_sequence(path: &Path) -> String {
    let input_file = File::open(&path).unwrap();
    let input_buffer = BufReader::new(input_file);
    input_buffer.lines().filter_map(|l| {
        let l = l.unwrap();
        match l.starts_with(">") {
            false => Some(l),
            true => None,
        }
    }).collect()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complement() {
        assert_eq!(complement('A'), 'T');
        assert_eq!(complement('C'), 'G');
        assert_eq!(complement('G'), 'C');
        assert_eq!(complement('T'), 'A');
        assert_eq!(complement('N'), 'N');
        assert_eq!(complement('?'), 'N');
    }

    #[test]
    fn test_reverse_complement() {
        assert_eq!(reverse_complement("ATAG"), "CTAT");
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome("ATAGCTAT"), true);
        assert_eq!(is_palindrome("ATAGCTAA"), false);
        assert_eq!(is_palindrome("ATAGCTA"), false);
        assert_eq!(is_palindrome("TGAGTCGATGCAAGGAAATAGAGCGCGCTCTATTTCCTTGCATCGACTCA"), true);
    }

    #[test]
    fn test_load_sequence() {
        let input_path = PathBuf::from(format!("{}/tests/tiny.fasta", var("CARGO_MANIFEST_DIR").unwrap()));
        assert_eq!(load_sequence(&input_path), "ACATGAGGC");
    }
}