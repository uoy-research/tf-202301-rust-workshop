# Building a Rust Program to Find Palindromes

We want to build a program that takes a nucleotide sequence and finds the location of all substrings of length *n* that are palindromic.

This program should take a FASTA file as input and find all palindromic sequences of length *n* using a sliding window across the entire sequence.

## Create the Rust Project

~~~bash
cargo init palindrome
cd palindrome
code -n .
cp -r /Users/apd500/Documents/Work/Talks/2023-rust-workshop/tests .
cargo run
~~~

## Set up the Repo

Add the files we don't want to track to `.gitignore`:

~~~plain
**/.DS_Store
/target
/tests/
!/tests/tiny.fasta
~~~

Make the first commit to the repo:

~~~bash
git add src/main.rs .gitignore Cargo.lock Cargo.toml
git commit -m"Initial commit of palindrome project"
~~~

## Create and Test the Reverse Complement Function

We need to add the function to our `main.rs` file:

~~~rust
fn complement(c: char) -> char {
    match c {
        'A' => 'T',
        'C' => 'G',
        'G' => 'C',
        'T' => 'A',
        _ => 'N',
    }
}
~~~

Add some testing:

~~~rust
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
        assert_eq!(complement('X'), 'N');
    }
}
~~~

Run the test then commit:

~~~bash
cargo test
git add src/main.rs
git commit -m"Add complement"
~~~

Create the reverse complement:

~~~rust
fn reverse_complement(s: &str) -> String {
    s.chars().map(|c| complement(c)).rev().collect()
}
~~~

And its test:

~~~rust
    #[test]
    fn test_reverse_complement() {
        assert_eq!(reverse_complement("ATAG"), "CTAT");
    }
~~~

Run the test then commit:

~~~bash
cargo test
git add src/main.rs
git commit -m"Add reverse_complement"
~~~

## Create and test the Palindrome function

~~~rust
fn is_palindrome(s: &str) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }
    let half_len = s.len() / 2_usize;
    s[0..half_len] == reverse_complement(&s[half_len..])
}
~~~

Add the test:

~~~rust
    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome("ATAGCTAT"), true);
        assert_eq!(is_palindrome("ATAGCTAA"), false);
        assert_eq!(is_palindrome("ATAGCTA"), false);
        assert_eq!(is_palindrome("TGAGTCGATGCAAGGAAATAGAGCGCGCTCTATTTCCTTGCATCGACTCA"), true);
    }
~~~

Run the tests:

~~~bash
cargo test
git add src/main.rs
git commit -m"Add is_palindrome"
~~~

## Load the Input Sequence

~~~rust
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader};

fn load_sequence(path: &Path) -> String {
    let input_file = File::open(&path).unwrap();
    let input_buffer = BufReader::new(input_file);
    input_buffer.lines().filter_map(|l| {
        let l = l.unwrap();
        match l.starts_with(">") {
            true => Some(l),
            false => None,
        }
    }).collect()
}
~~~

Add a test:

~~~rust
use std::path::{Path, PathBuf};

#[test]
fn test_load_sequence() {
    let input_path = PathBuf::from(format!("{}/tests/tiny.fasta", var("CARGO_MANIFEST_DIR").unwrap()));
    assert_eq!(load_sequence(&input_path), "ACATGAGGC");
}
~~~

Run the tests:

~~~bash
cargo test
git add src/main.rs
git commit -m"Add load_sequence"
~~~

## Build the main function

~~~rust
let window_size:usize = 100;
let input_path = PathBuf::from(format!("{}/tests/small.fasta", var("CARGO_MANIFEST_DIR").unwrap()));
let input_sequence = load_sequence(&input_path);
let positions:Vec<usize> = (0_usize..(input_sequence.len() - window_size)).into_iter().filter_map(|i| {
    let slice = &input_sequence[i..(i + window_size)];
    match is_palindrome(slice) {
        false => None,
        true => Some(i),
    }
}).collect();
for i in positions {
    println!("{}\t{}", i, &input_sequence[i..(i + window_size)]);
}
~~~

~~~bash
cargo run
git add src/main.rs
git commit -m"Add main loop"
~~~

## Build and Test the all N removal

~~~rust
fn all_n(s: &str) -> bool {
    s.chars().all(|c| c == 'N')
}

#[test]
fn test_all_n() {
    assert_eq!(all_n("NNNANNN"), false);
    assert_eq!(all_n("NNNNNNN"), true);
}
~~~

~~~bash
cargo test
~~~

## Update to strip out all N sequences

~~~rust
if all_n(&slice) {
    return None
};
~~~

~~~bash
git add src/main.rs
git commit -m"Add all_n"
~~~

## Remove the Need to Make a new String

We can update is_palindrome to no longer call reverse_complement, and so never make a new String

~~~rust
fn is_palindrome(s: &str) -> bool {
    s.chars().zip(s.chars().rev()).all(|(i, j)|{
        i == complement(j)
    })
}
~~~

We can also delete `reverse_complement` and its test.

~~~bash
cargo test
git add src/main.rs
git commit -m"Update to prevent new String creation"
~~~

## Update to Rayon

~~~bash
cargo add rayon
~~~

~~~rust
use rayon::prelude::*;
rayon::ThreadPoolBuilder::new().num_threads(8).build_global().unwrap();
~~~

~~~bash
git add src/main.rs Cargo.toml Cargo.lock
git commit -m"Convert to multi-threading"
~~~

## Check times

~~~bash
time cargo run --release
~~~

## Show Clippy

~~~bash
cargo clippy
cargo fmt
git add src/main.rs
git commit -m"Clippy and formatter"
~~~

## Try against the whole genome

Update the input path to `genome.fasta`

~~~bash
cargo build --release
cargo run --release > palindromes.txt
~~~

## Issues with the Code

* We find palindromes across FASTA sequence boundaries
* We don't account for mismatches
* We include known repetitive sequences (like poly-`AT`)
* We need to re-build the code to change the input file
* We don't do any sensible error handling

## Add the CLI

Get the Clap crate

~~~bash
cargo add clap -Fderive
~~~

Add the CLI:

~~~rust
use clap::Parser;

#[derive(Parser)]
#[command(version)]
/// Find palindromes in FASTA files
struct Args {
    /// The desired palindrome length
    #[clap(short = 'l', long = "length", value_name = "l", default_value = "10")]
    length: usize,
    /// The number of threads to use
    #[clap(short = 't', long = "threads", value_name = "n", default_value = "1")]
    threads: usize,
    /// The input FASTA file
    #[clap(value_name = "FILE")]
    input_path: PathBuf,
}
~~~

In main, we need to make some changes:

~~~rust
let args = Args::parse();
rayon::ThreadPoolBuilder::new()
    .num_threads(args.threads)
    .build_global()
    .unwrap();
let window_size = args.length;
let input_sequence = load_sequence(&args.input_path);
~~~

Now that we don't use `std::env::var` in the main code, we can move it to the test section.

~~~bash
git add src/main.rs Cargo.lock Cargo.toml
git commit -m"Add CLI"
~~~

## Add Error Handling

We have used `unwrap()` throughout the code. When code calls unwrap, errors will cause an ugly program panic.  We can see this by supplying an invalid file

~~~bash
cargo run --release -- ./tests/nope.fasta
~~~

We can tidy this up by catching and handling errors.  We'l use a crate called simple-eyre for this here.

~~~bash
cargo add simple-eyre
~~~

We now add the new code, update the main function signature, and add the main error handler:

~~~rust
use simple_eyre::eyre::Result;

fn main -> Result<()> {
    simple_eyre::install()?;
    ...
    Ok(())
}
~~~

Once we've done that, we can replace all the `unwrap()` calls with `?`. This will require changing the `load_sequence` to return a result enum:

~~~rust
fn load_sequence(path: &Path) -> Result<String> {
    let input_file = File::open(path)?;
    let input_buffer = BufReader::new(input_file);
    Ok(input_buffer
        .lines()
        .filter_map(|l| match l {
            Err(_) => None,
            Ok(line) if line.starts_with('>') => None,
            Ok(line) => Some(line),
        })
        .collect())
}
~~~

After this change, we need to update our tests to accept the new signature.

~~~bash
cargo test
cargo clippy
cargo fmt
git add src/main.rs Cargo.toml Cargo.lock
git commit -m"Update to handle errors"
~~~

## We can now use the compiled binary directly without the use of the rest of the directory

~~~bash
cargo build --release
./target/release/palindrome --help
~~~
