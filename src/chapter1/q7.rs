// 1.7 - Rotate Matrix
#![allow(dead_code)]

const SIZE: usize = 4;

fn rotate_matrix(matrix: &mut [[usize; SIZE]; SIZE]) {
    let n = matrix.len();

    for i in 0..n {
        for j in i..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    for row in matrix.iter_mut() {
        row.reverse();
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter1::q7::rotate_matrix;

    #[test]
    fn should_rotate() {
        let mut matrix = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ];
        let expected = [
            [13, 9, 5, 1],
            [14, 10, 6, 2],
            [15, 11, 7, 3],
            [16, 12, 8, 4],
        ];

        rotate_matrix(&mut matrix);
        assert_eq!(matrix, expected)
    }
}
