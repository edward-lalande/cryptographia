//
// EPITECH PROJECT, 2024
// mod.rs
// File description:
// import modules
//

use matrix::Matrix;

pub mod aes;
pub mod cipher_message;
pub mod decipher_message;
pub mod key_expansion;
pub mod add_round_key;
pub mod rot_word;
pub mod sub_word;
pub mod shift_rows;
pub mod sub_bytes;
pub mod r_con;
pub mod mix_columns;
pub mod matrix;

type Word = Vec<u8>;
type State = Matrix;
pub const ROUNDS: [(usize, usize); 3] = [
    (16, 11),
    (24, 13),
    (32, 15),
];
