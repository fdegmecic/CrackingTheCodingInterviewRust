// 1.6 - String Compression
#![allow(dead_code)]

fn compress_string(string: &str) -> String {
    let chars = string.chars().collect::<Vec<char>>();
    let mut current_char = chars.first().unwrap();
    let mut iteration = 0;
    let mut count = 0;
    let mut result = String::new();
    for c in &chars {
        if c == current_char {
            count += 1;
        }
        if c != current_char || chars.len() - 1 == iteration {
            result.push(*current_char);
            result.push_str(&count.to_string());
            count = 1;
        }

        iteration += 1;
        current_char = c;
    }

    if result.len() > string.len() {
        return String::from(string);
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::chapter1::q6::compress_string;

    #[test]
    fn should_compress_string() {
        assert_eq!(compress_string("aabcccccaaa"), String::from("a2b1c5a3"));
    }

    #[test]
    fn should_not_compress_string() {
        assert_eq!(compress_string("abca"), String::from("abca"));
    }
}
