//1.2 - Check Permutation
#![allow(dead_code)]

fn is_permutation_brute_force(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false;
    }

    sort_str(string1) == sort_str(string2)
}

fn sort_str(str: &str) -> Vec<char> {
    let mut chars = str.chars().collect::<Vec<_>>();
    chars.sort();
    chars.into()
}

trait LowercaseLetter {
    fn to_u32_for_bitset(&self) -> u32;
}

impl LowercaseLetter for u8 {
    fn to_u32_for_bitset(&self) -> u32 {
        assert!(self.is_ascii_lowercase());
        1 << (*self as u32 - 'a' as u32)
    }
}

fn is_permutation_bitset(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false;
    }

    let bitset1 = string1
        .as_bytes()
        .iter()
        .map(|c| c.to_u32_for_bitset())
        .fold(0, |acc, x| acc | x);
    let bitset2 = string2
        .as_bytes()
        .iter()
        .map(|c| c.to_u32_for_bitset())
        .fold(0, |acc, x| acc | x);

    bitset1 == bitset2
}

fn is_permutation(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false;
    }

    let mut frequency = [0; 128];
    for char in string1.chars() {
        frequency[char as usize] += 1;
    }

    for char in string2.chars() {
        frequency[char as usize] -= 1;
        if frequency[char as usize] < 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::chapter1::q2::*;

    #[test]
    fn should_be_permutation() {
        assert_eq!(true, is_permutation_brute_force("cat", "tac"));
        assert_eq!(true, is_permutation_bitset("cat", "tac"));
        assert_eq!(true, is_permutation("cat", "tac"));
    }

    #[test]
    fn should_not_be_permutation() {
        assert_eq!(false, is_permutation_brute_force("cat", "dog"));
        assert_eq!(false, is_permutation_bitset("cat", "dog"));
        assert_eq!(false, is_permutation("cat", "dog"));
    }
}
