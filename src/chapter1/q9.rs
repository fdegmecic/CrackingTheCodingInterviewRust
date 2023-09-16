// 1.9 - String Rotation
#![allow(dead_code)]

fn is_rotation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    return is_substring(s1, s1);
}

fn is_substring(s1: &str, s2: &str) -> bool {
    s1.contains(s2)
}

#[cfg(test)]
mod tests {
    use crate::chapter1::q9::is_rotation;

    #[test]
    fn should_be_rotation() {
        assert_eq!(is_rotation("waterbottle", "erbottlewat"), true);
    }

    #[test]
    fn should_not_be_rotation() {
        assert_eq!(is_rotation("true", "false"), false);
    }
}
