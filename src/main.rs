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
}