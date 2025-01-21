//
// EPITECH PROJECT, 2024
// key_expansion.rs
// File description:
// Builds subkeys from the original key
//

use crate::aes::r_con::RCON;
use crate::aes::rot_word::rot_word;
use crate::aes::sub_word::sub_word;
use super::matrix::Matrix;
use super::{State, Word, ROUNDS};

pub fn key_expansion(cipher_key: &[u8]) -> Option<State> {
    let tmp = ROUNDS.iter().find(|r| r.0 == cipher_key.len());
    if tmp.is_none() {
        eprintln!("Key size is not 128 bits.");
        return None;
    }
    let nb_rounds = tmp.unwrap().1;
    let matrix_key = Matrix::from_shape(cipher_key, (4, 4));
    let mut state = Matrix::new((4, 4 * nb_rounds));
    for i in 0..state.rows {
        for j in 0..state.rows {
            state[i][j] = matrix_key[j][i];
        }
    }

    for i in 4..(4 * nb_rounds) {
        let mut word: Word = state.get_col(i - 1)?.iter().map(|&&c| c).collect();
        if i % 4 == 0 {
            rot_word(&mut word, 1);
            sub_word(&mut word);
            word[0] ^= RCON[(i / 4) - 1];
        }
        let prev_word: Word = state.get_col(i - 4)?.iter().map(|&&c| c).collect();
        for j in 0..state.rows {
            word[j] ^= prev_word[j];
            state[j][i] = word[j];
        }
    }
    Some(state)
}
