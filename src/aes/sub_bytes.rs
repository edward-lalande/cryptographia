//
// EPITECH PROJECT, 2024
// sub_bytes.rs
// File description:
// Applu sub_word() to every word of a block
//

use super::{sub_word::{inv_sub_word, sub_word}, State};

pub fn inv_sub_bytes(key: &mut State) {
    for i in 0..key.rows {
        inv_sub_word(key.get_row_mut(i).unwrap());
    }
}

pub fn sub_bytes(key: &mut State) {
    for i in 0..key.rows {
        sub_word(key.get_row_mut(i).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::aes::{matrix::Matrix, sub_bytes::inv_sub_bytes};
    use super::sub_bytes;

    #[test]
    fn test_sub_bytes() {
        let mut state = Matrix::from_matrix_array(&[
            [0x67, 0x61, 0x6d, 0x65],
            [0x20, 0x6f, 0x66, 0x20],
            [0x74, 0x68, 0x72, 0x6f],
            [0x6e, 0x65, 0x73, 0x0a]
        ]);
        sub_bytes(&mut state);
        assert_eq!(state.data, [
            [0x85, 0xef, 0x3c, 0x4d],
            [0xb7, 0xa8, 0x33, 0xb7],
            [0x92, 0x45, 0x40, 0xa8],
            [0x9f, 0x4d, 0x8f, 0x67]
        ]);
    }

    #[test]
    fn test_inv_sub_bytes() {
        let mut state = Matrix::from_matrix_array(&[
            [0x85, 0xef, 0x3c, 0x4d],
            [0xb7, 0xa8, 0x33, 0xb7],
            [0x92, 0x45, 0x40, 0xa8],
            [0x9f, 0x4d, 0x8f, 0x67]
        ]);
        inv_sub_bytes(&mut state);
        assert_eq!(state.data, [
            [0x67, 0x61, 0x6d, 0x65],
            [0x20, 0x6f, 0x66, 0x20],
            [0x74, 0x68, 0x72, 0x6f],
            [0x6e, 0x65, 0x73, 0x0a]
        ]);
    }
}
