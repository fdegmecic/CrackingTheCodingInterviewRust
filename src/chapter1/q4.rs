//1.4 - Palindrome Permutation
#![allow(dead_code)]

fn is_permutation(string: &str) -> bool {
    let mut frequency = vec![0; 'z' as usize - 'a' as usize + 1];
    let mut count = 0;
    for c in string.chars() {
        if c == ' ' {
            continue;
        }

        let x = 'z' as usize - c as usize;
        frequency[x] += 1;
        if frequency[x] % 2 == 1 {
            count += 1;
        } else {
            count -= 1;
        }
    }

    count <= 1
}

#[cfg(test)]
mod tests {
    use crate::chapter1::q4::is_permutation;

    #[test]
    fn should_be_palindome_permutation() {
        assert_eq!(is_permutation("taco cat"), true);
    }

    #[test]
    fn should_not_be_palindome_permutation() {
        assert_eq!(is_permutation("taco catt"), false);
    }
}
