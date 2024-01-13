// 5.3 - Flip Bit to Win
#![allow(dead_code)]

fn find_longest_sequence(mut bits: u32) -> usize {
    let (mut curr, mut prev, mut max) = (0, 0, 0);

    while bits != 0 {
        if (bits & 1) == 1 {
            curr += 1;
        } else if (bits & 1) == 0 {
            if (bits & 2) == 0 {
                prev = 0
            } else {
                prev = curr;
            }
            curr = 0;
        }

        max = (prev + curr + 1).max(max);
        bits >>= 1;
    }

    max
}

#[cfg(test)]
mod tests {
    use crate::chapter5::q3::find_longest_sequence;

    #[test]
    fn should_find_longest_sequence() {
        assert_eq!(find_longest_sequence(0b110111110111), 9);
        assert_eq!(find_longest_sequence(0b11101111101), 9);
    }
}
