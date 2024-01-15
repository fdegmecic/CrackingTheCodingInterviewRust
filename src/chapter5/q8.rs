// 5.8 - Draw Line
#![allow(dead_code)]

use std::fmt::Display;

struct Screen<const N: usize> {
    array: [u8; N],
    width: usize,
}

impl<const N: usize> Screen<N> {
    pub fn new(width: usize) -> Self {
        assert!(N % width == 0);
        Screen {
            array: [0; N],
            width,
        }
    }
}

impl<const N: usize> Display for Screen<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.array.iter();
        let mut dividor = None;

        while iter.len() != 0 {
            let mut row = Vec::new();
            for _ in 0..self.width {
                row.push(format!("{:08b}", iter.next().unwrap()));
            }
            let row = "| ".to_owned() + &row.join(" | ") + " |";

            dividor = dividor.or_else(|| Some(String::from("-").repeat(row.len())));

            writeln!(f, "{}", dividor.as_ref().unwrap()).ok();
            writeln!(f, "{}", row).ok();
        }

        if let Some(dividor) = dividor {
            return writeln!(f, "{}", dividor);
        }
        writeln!(f, "Empty screen")
    }
}

fn draw_line<const N: usize>(screen: &mut Screen<N>, width: usize, x1: usize, x2: usize, y: usize) {
    let row_start = width * y;
    let row = &mut screen.array[row_start + (x1 / 8)..=row_start + (x2 / 8)];

    let first_mask = !0 >> (x1 % 8) as u8;
    let last_mask = !(!0 >> ((x2 + 1) % 8));

    if let Some(first) = row.first_mut() {
        if x2 / 8 == x1 / 8 {
            *first = last_mask & first_mask;
            return;
        }
        *first = first_mask;
    }

    if let Some(last) = row.last_mut() {
        *last = last_mask;
    }

    if row.len() > 2 {
        let middle_end = row.len() - 1;
        for middle in &mut row[1..middle_end] {
            *middle = !0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_draw_line() {
        let width = 3;
        let mut screen = Screen::<6>::new(width);
        draw_line(&mut screen, width, 8, 16, 0);
        println!("{}", screen);
        assert_eq!(
            format!("{}", screen),
            "----------------------------------
| 00000000 | 11111111 | 10000000 |
----------------------------------
| 00000000 | 00000000 | 00000000 |
----------------------------------
"
        );
    }
}
