//
// EPITECH PROJECT, 2024
// rot_word.rs
// File description:
// Takes a 4-bytes word, and shifts cyclically every byte to the left once
//

use super::Word;

pub fn rot_word(word: &mut Word, mid: usize) {
    word.rotate_left(mid);
}
