#![allow(dead_code)]
use std::{collections::HashSet, vec};

fn has_unique_brute_force(string: &str) -> bool {
    for char1 in string.chars() {
        for char2 in string.chars() {
            if char1 == char2 {
                return false;
            }
        }
    }

    true
}

fn has_unqiue_hash_set(string: &str) -> bool {
    string.chars().into_iter().collect::<HashSet<_>>().len() == string.len()
}

fn has_unique_hash_set_manually(string: &str) -> bool {
    let mut hash_set = HashSet::new();

    for char in string.chars() {
        if !hash_set.insert(char) {
            return false;
        }
    }

    return true;
}

fn has_unique_vector(string: &str) -> bool {
    let mut vec = Vec::with_capacity(string.len());

    for char in string.chars() {
        if vec.contains(&char) {
            return false;
        }

        vec.push(char);
    }

    return true;
}

fn has_unique_array(string: &str) -> bool {
    let mut arr = vec![0u8; string.len()];
    let mut index = 0;

    for char in string.chars() {
        for i in 0..index {
            if arr[i] == char as u8 {
                return false;
            }
        }

        arr[index] = char as u8;
        index += 1;
    }

    return true;
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

fn has_unique_bit_manipulation(string: &str) -> bool {
    string
        .as_bytes()
        .iter()
        .map(|c| c.to_u32_for_bitset())
        .fold(0, |acc, x| acc | x)
        .count_ones()
        == string.len() as u32
}

#[cfg(test)]
mod test {
    use crate::chapter1::q1::*;

    #[test]
    fn should_be_unqiue() {
        assert_eq!(true, has_unqiue_hash_set("string"));
        assert_eq!(true, has_unique_hash_set_manually("string"));
        assert_eq!(true, has_unique_vector("string"));
        assert_eq!(true, has_unique_array("string"));
        assert_eq!(true, has_unique_bit_manipulation("string"));
    }

    #[test]
    fn should_not_be_unqiue() {
        assert_eq!(false, has_unqiue_hash_set("banana"));
        assert_eq!(false, has_unique_hash_set_manually("banana"));
        assert_eq!(false, has_unique_vector("banana"));
        assert_eq!(false, has_unique_array("banana"));
        assert_eq!(false, has_unique_bit_manipulation("banana"));
    }
}
