// 5.7 - Pairwise Swap
#![allow(dead_code)]

fn pairwise_swap(n: u32) -> u32 {
    let evens = n & 0x5555_5555;
    let odds = n & 0xAAAA_AAAA;
    (odds >> 1) | (evens << 1)
}

#[cfg(test)]
mod tests {
    use crate::chapter5::q7::pairwise_swap;

    #[test]
    fn should_pairwise_swap() {
        assert_eq!(pairwise_swap(123), 183)
    }
}
