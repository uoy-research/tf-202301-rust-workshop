use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use clap::Parser;

#[derive(Parser)]
#[command(version)]
/// Find palindromes in FASTA files
struct Args {
    /// Show log messages. Multiple -v options increase the verbosity
    #[clap(short='v', long="verbose", action=clap::ArgAction::Count)]
    verbose: u8,
    /// The desired palindrome length
    #[clap(short = 'l', long = "length", value_name = "len", default_value = "10")]
    length: usize,
    /// The desired thread number
    #[clap(short = 't', long = "threads", value_name = "n", default_value = "1")]
    threads: usize,
    /// The input FASTA file
    #[clap(value_name = "FILE")]
    input_path: PathBuf,
}

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
    let args = Args::parse();
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .unwrap();
    let window_size = args.length;
    let input_sequence = load_sequence(&args.input_path);
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
    for i in positions {
        println!("{}\t{}", i, &input_sequence[i..(i + window_size)]);
    }
}

#[cfg(test)]
mod tests {
    use std::env::var;
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
