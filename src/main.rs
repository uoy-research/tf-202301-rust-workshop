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
}