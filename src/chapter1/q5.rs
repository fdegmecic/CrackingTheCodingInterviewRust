//1.5 - One Away
#![allow(dead_code)]

fn is_one_away(str1: &str, str2: &str) -> bool {
    if str1.len() == str2.len() {
        one_edit_replace(str1, str2)
    } else if str1.len() + 1 == str2.len() {
        one_edit_insert(str1, str2)
    } else if str1.len() - 1 == str2.len() {
        one_edit_insert(str2, str1)
    } else {
        false
    }
}

fn one_edit_replace(str1: &str, str2: &str) -> bool {
    let mut found_difference = false;
    for i in 0..str1.len() {
        if str1.chars().nth(i) != str2.chars().nth(i) {
            if found_difference {
                return false;
            }

            found_difference = true
        }
    }

    true
}

fn one_edit_insert(str1: &str, str2: &str) -> bool {
    let (s, l) = get_ordered(str1, str2);
    let mut iter = 0;

    let longer: Vec<char> = l.chars().collect();
    let shorter: Vec<char> = s.chars().collect();

    while iter < shorter.len() {
        if longer[iter] != shorter[iter] {
            return longer[iter + 1..] == shorter[iter..];
        }

        iter += 1;
    }

    true
}

fn get_ordered<'a>(s1: &'a str, s2: &'a str) -> (&'a str, &'a str) {
    if s1.len() < s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    }
}

#[cfg(test)]
mod test {
    use crate::chapter1::q5::is_one_away;

    #[test]
    fn should_be_one_away() {
        assert_eq!(is_one_away("pale", "ple"), true);
        assert_eq!(is_one_away("ple", "pale"), true);
        assert_eq!(is_one_away("pale", "bale"), true);
        assert_eq!(is_one_away("pales", "pale"), true);
    }

    #[test]
    fn should_not_be_one_away() {
        assert_eq!(is_one_away("pale", "bake"), false);
    }
}
