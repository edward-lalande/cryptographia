//
// EPITECH PROJECT, 2024
// shift_rows.rs
// File description:
// Shift cyclically every row of a 4x4 matric (a block) by 0, 1, 2 or 3 times, for the 1st, 2nd, 3rd and 4th row respectively
//

use super::State;

pub fn inv_shift_rows(key: &mut State) {
    for i in 1..key.rows as i32 {
        let row = [key[i as usize][((0 - i + 4) % 4) as usize], key[i as usize][((1 - i + 4) % 4) as usize], key[i as usize][((2 - i + 4) % 4) as usize], key[i as usize][((3 - i + 4) % 4) as usize]];
        for j in 0..key.rows {
            key[i as usize][j] = row[j];
        }
    }
}

pub fn shift_rows(key: &mut State) {
    for i in 1..key.rows {
        let row = [key[i][(0 + i) % 4], key[i][(1 + i) % 4], key[i][(2 + i) % 4], key[i][(3 + i) % 4]];
        for j in 0..key.rows {
            key[i][j] = row[j];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::aes::{matrix::Matrix, shift_rows::inv_shift_rows};
    use super::shift_rows;

    #[test]
    fn test_shift_rows() {
        let mut state = Matrix::from_matrix_array(&[
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16]
        ]);
        shift_rows(&mut state);
        assert_eq!(state.data, [
            [1, 2, 3, 4],
            [6, 7, 8, 5],
            [11, 12, 9, 10],
            [16, 13, 14, 15]
        ]);
    }

    #[test]
    fn test_inv_shift_rows() {
        let mut state = Matrix::from_matrix_array(&[
            [1, 2, 3, 4],
            [6, 7, 8, 5],
            [11, 12, 9, 10],
            [16, 13, 14, 15]
        ]);
        inv_shift_rows(&mut state);
        assert_eq!(state.data, [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16]
        ]);
    }
}
