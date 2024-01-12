// 5.2 - Binary to String
#![allow(dead_code)]

pub fn decimal_to_binary(mut f: f64) -> String {
    if f <= 0.0 || f >= 1.0 {
        panic!("Invalid Inputs");
    }

    let mut result = String::from("0.");
    let mut comp = 0.5;
    while f > 0.0 {
        if result.len() > 32 + 2 {
            panic!("ERROR");
        }
        if f >= comp {
            result.push('1');
            f -= comp;
        } else {
            result.push('0');
        }
        comp /= 2.0;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::chapter5::q2::decimal_to_binary;

    #[test]
    fn should_print_decimal_to_binary() {
        //assert_eq!(decimal_to_binary(0.5), "0.1".to_string());
        assert_eq!(decimal_to_binary(0.25), "0.01".to_string());
        assert_eq!(decimal_to_binary(0.125), "0.001".to_string());
        assert_eq!(decimal_to_binary(0.875), "0.111".to_string());
        assert_eq!(decimal_to_binary(0.6252), "0.101".to_string());
    }
}
