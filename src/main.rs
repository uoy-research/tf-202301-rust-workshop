use rayon::prelude::*;
use std::env::var;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

fn complement(c: char) -> char {
    match c {
        'A' => 'T',
        'C' => 'G',
        'G' => 'C',
        'T' => 'A',
        _ => 'N',
    }
}

fn is_palindrome(s: &str) -> bool {
    s.chars()
        .zip(s.chars().rev())
        .all(|(i, j)| i == complement(j))
}

fn all_n(s: &str) -> bool {
    s.chars().all(|c| c == 'N')
}

fn load_sequence(path: &Path) -> String {
    let input_file = File::open(path).unwrap();
    let input_buffer = BufReader::new(input_file);
    input_buffer
        .lines()
        .filter_map(|l| {
            let l = l.unwrap();
            match l.starts_with('>') {
                false => Some(l),
                true => None,
            }
        })
        .collect()
}

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build_global()
        .unwrap();
    let window_size: usize = 100;
    let input_path = PathBuf::from(format!(
        "{}/tests/chr1.fasta",
        var("CARGO_MANIFEST_DIR").unwrap()
    ));
    let input_sequence = load_sequence(&input_path);
    let positions: Vec<usize> = (0_usize..(input_sequence.len() - window_size))
        .into_par_iter()
        .filter_map(|i| {
            let slice = &input_sequence[i..(i + window_size)];
            if all_n(slice) {
                return None;
            };
            match is_palindrome(slice) {
                false => None,
                true => Some(i),
            }
        })
        .collect();
    println!("{} palindromes found", positions.len());
    for i in positions {
        println!("{}\t{}", i, &input_sequence[i..(i + window_size)]);
    }
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
    fn test_is_palindrome() {
        assert_eq!(is_palindrome("ATAGCTAT"), true);
        assert_eq!(is_palindrome("ATAGCTAA"), false);
        assert_eq!(is_palindrome("ATAGCTA"), false);
        assert_eq!(
            is_palindrome("TGAGTCGATGCAAGGAAATAGAGCGCGCTCTATTTCCTTGCATCGACTCA"),
            true
        );
    }

    #[test]
    fn test_load_sequence() {
        let input_path = PathBuf::from(format!(
            "{}/tests/tiny.fasta",
            var("CARGO_MANIFEST_DIR").unwrap()
        ));
        assert_eq!(load_sequence(&input_path), "ACATGAGGC");
    }

    #[test]
    fn test_all_n() {
        assert_eq!(all_n("NNNNN"), true);
        assert_eq!(all_n("NNTNN"), false);
    }
}
