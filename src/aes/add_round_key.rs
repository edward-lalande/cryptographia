//
// EPITECH PROJECT, 2024
// add_round_key.rs
// File description:
// XOR with the subkey
//

use super::State;

pub fn add_round_key(state: &mut State, keys: &State, round: usize) {
    for i in 0..state.rows {
        for j in (round * 4)..(round * 4 + 4) {
            state[i][j - (round * 4)] ^= keys[i][j];
        }
    }
}
