//
// EPITECH PROJECT, 2024
// mix_columns.rs
// File description:
// Takes a word (4 bytes) and performs a matrix multiplication with a const matrix
//

use super::{matrix::Matrix, State};

fn gmul_2(x: u8) -> u8 {
    if (x & 0x80) != 0 {
        (x << 1) ^ 0x1b
    } else {
        x << 1
    }
}

fn gmul_3(x: u8) -> u8 {
    gmul_2(x) ^ x
}

fn gmul_9(x: u8) -> u8 {
    gmul_2(gmul_2(gmul_2(x))) ^ x
}

fn gmul_11(x: u8) -> u8 {
    gmul_2(gmul_2(gmul_2(x)) ^ x) ^ x
}

fn gmul_13(x: u8) -> u8 {
    gmul_2(gmul_2(gmul_2(x) ^ x)) ^ x
}

fn gmul_14(x: u8) -> u8 {
    gmul_2(gmul_2(gmul_2(x) ^ x) ^ x)
}

fn gmul(x: u8, mul: u8) -> u8 {
    match mul {
        1 => x,
        2 => gmul_2(x),
        3 => gmul_3(x),
        9 => gmul_9(x),
        11 => gmul_11(x),
        13 => gmul_13(x),
        14 => gmul_14(x),
        _ => panic!("Multiplier is not supported for AES encryption."),
    }
}

fn get_mixed_word(col: &mut Vec<&mut u8>, calc_matrix: &Matrix) -> [u8; 4] {
    let mut word = [0; 4];

    for j in 0..word.len() {
        let mut val = 0;
        for k in 0..word.len() {
            val ^= gmul(*col[k], calc_matrix[j][k]);
        }
        word[j] = val;
    }
    word
}

pub fn inv_mix_columns(key: &mut State) {
    let calc_matrix = Matrix::from_matrix_array(&[
        [14, 11, 13, 9],
        [9, 14, 11, 13],
        [13, 9, 14, 11],
        [11, 13, 9, 14]
    ]);

    for i in 0..key.rows {
        if let Some(mut col) = key.get_col_mut(i) {
            let word = get_mixed_word(&mut col, &calc_matrix);
            for j in 0..word.len() {
                *col[j] = word[j];
            }
        } else {
            panic!("key.get_col_mul({i}) returned None.");
        }
    }
}

pub fn mix_columns(key: &mut State) {
    let calc_matrix = Matrix::from_matrix_array(&[
        [2, 3, 1, 1],
        [1, 2, 3, 1],
        [1, 1, 2, 3],
        [3, 1, 1, 2]
    ]);

    for i in 0..key.rows {
        if let Some(mut col) = key.get_col_mut(i) {
            let word = get_mixed_word(&mut col, &calc_matrix);
            for j in 0..word.len() {
                *col[j] = word[j];
            }
        } else {
            panic!("key.get_col_mul({i}) returned None.");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::aes::{matrix::Matrix, mix_columns::inv_mix_columns};

    use super::mix_columns;

    #[test]
    fn test_mix_columns() {
        let mut key = Matrix::from_matrix_array(&[
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0xbf, 0xb4, 0x41, 0x27],
            [0x5d, 0x52, 0x11, 0x98],
            [0x30, 0xae, 0xf1, 0xe5]
        ]);
        mix_columns(&mut key);
        let cmp = Matrix::from_matrix_array(&[
            [0x04, 0xe0, 0x48, 0x28],
            [0x66, 0xcb, 0xf8, 0x06],
            [0x81, 0x19, 0xd3, 0x26],
            [0xe5, 0x9a, 0x7a, 0x4c]
        ]);
        assert_eq!(key, cmp);
    }

    #[test]
    fn test_inv_mix_columns() {
        let mut key = Matrix::from_matrix_array(&[
            [0x04, 0xe0, 0x48, 0x28],
            [0x66, 0xcb, 0xf8, 0x06],
            [0x81, 0x19, 0xd3, 0x26],
            [0xe5, 0x9a, 0x7a, 0x4c]
        ]);
        inv_mix_columns(&mut key);
        let cmp = Matrix::from_matrix_array(&[
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0xbf, 0xb4, 0x41, 0x27],
            [0x5d, 0x52, 0x11, 0x98],
            [0x30, 0xae, 0xf1, 0xe5]
        ]);
        assert_eq!(key, cmp);
    }
}
