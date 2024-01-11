// 5.1 - Insertion
#![allow(dead_code)]

fn insert_bits(n: u32, m: u32, i: u32, j: u32) -> u32 {
    let j_thru_i = (2_u32.pow(j) - 1) ^ (2_u32.pow(i) - 1);
    let mut rv = n & !j_thru_i;
    rv |= m << i;

    rv
}

#[cfg(test)]
mod tests {
    use super::insert_bits;

    #[test]
    fn should_insert_bits() {
        let result = insert_bits(int("10000000000"), int("10011"), 2, 6);
        assert_eq!(result, int("10001001100"));
    }

    fn int(s: &str) -> u32 {
        u32::from_str_radix(s, 2).unwrap()
    }
}
