//1.3 - URLify
#![allow(dead_code)]

fn urlify(string: &str) -> String {
    return string.trim_end().replace(" ", "%20");
}

fn urlify_array(string: &str) -> String {
    let mut result = String::new();
    let string: Vec<&str> = string.split_whitespace().collect();

    for (i, s) in string.iter().enumerate() {
        result.push_str(s);

        if i != string.len() - 1 {
            result.push_str("%20");
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::chapter1::q3::{urlify, urlify_array};

    #[test]
    fn should_urlify() {
        assert_eq!("Mr%20John%20Smith".to_string(), urlify("Mr John Smith    "));
        assert_eq!(
            "Mr%20John%20Smith".to_string(),
            urlify_array("Mr John Smith    ")
        );
    }
}
