// 5.4 - Next Number
#![allow(dead_code)]

fn get_next(num: usize) -> (usize, usize) {
    let ones = num.count_ones();

    let mut low_value = 0;
    {
        let mut low = num;
        while low > 0 {
            low -= 1;
            if low.count_ones() == ones {
                low_value = low;
                break;
            }
        }
    }

    let mut high_value = 0;
    {
        let mut high = num;
        while high < usize::MAX {
            high += 1;
            if high.count_ones() == ones {
                high_value = high;
                break;
            }
        }
    }

    (low_value, high_value)
}

#[cfg(test)]
mod tests {
    use crate::chapter5::q4::get_next;

    #[test]
    fn should_get_next() {
        assert_eq!(get_next(5), (3, 6));
        assert_eq!(get_next(4), (2, 8));
    }
}
