//1.5 - One Away
#![allow(dead_code)]

fn is_one_away(str1: &str, str2: &str) -> bool {
    if ((str1.len() as i32) - (str2.len() as i32)).abs() > 1 {
        return false;
    };

    one_edit_insert_replace(str1, str2)
}

fn one_edit_insert_replace(str1: &str, str2: &str) -> bool {
    let (s, l) = get_ordered(str1, str2);
    let mut iter = 0;

    let longer: Vec<char> = l.chars().collect();
    let shorter: Vec<char> = s.chars().collect();

    while iter < shorter.len() {
        if longer[iter] != shorter[iter] {
            let offset = match s.len() == l.len() {
                true => 1,
                false => 0,
            };
            return longer[iter + 1..] == shorter[iter + offset..];
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
