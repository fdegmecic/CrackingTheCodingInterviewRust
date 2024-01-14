// 5.6 - Conversion
#![allow(dead_code)]

fn count_bit_swaps(a: u32, b: u32) -> u32 {
    (a ^ b).count_ones()
}

#[cfg(test)]
mod tests {
    use crate::chapter5::q6::count_bit_swaps;

    #[test]
    fn should_count_required_bit_swaps() {
        assert_eq!(count_bit_swaps(29, 15), 2)
    }
}
