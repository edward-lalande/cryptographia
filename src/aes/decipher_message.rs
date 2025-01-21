//
// EPITECH PROJECT, 2024
// cipher_message.rs
// File description:
// AES deciphering
//

use crate::aes::{key_expansion::key_expansion, matrix::Matrix};

use super::{add_round_key::add_round_key, mix_columns::inv_mix_columns, shift_rows::inv_shift_rows, sub_bytes::inv_sub_bytes, State};

fn decrypt_block(mut state: &mut State, keys: &State) -> Option<bool> {
    let nb_rounds = keys.col_size / 4;

    add_round_key(&mut state, &keys, nb_rounds - 1);
    for round in (0..=(nb_rounds - 2)).rev() {
        inv_shift_rows(&mut state);
        inv_sub_bytes(&mut state);
        add_round_key(&mut state, &keys, round);
        if round != 0 {
            inv_mix_columns(&mut state);
        }
    }
    Some(true)
}

pub fn decipher_message(message: &[u8], cipher_key: &[u8]) -> Option<String> {
    let keys = key_expansion(cipher_key)?;
    let mut deciphered_text: Vec<u8> = vec![];
    for chunk in message.chunks(16) {
        let mut state = Matrix::from_key(chunk, (4, 4));
        decrypt_block(&mut state, &keys)?;
        deciphered_text.extend(state.flatten_cols());
    }
    Some(String::from_utf8_lossy(deciphered_text.as_slice()).trim_end_matches(|c| c == '\0').to_string())
}

#[cfg(test)]
mod tests {
    use super::decipher_message;

    #[test]
    fn test_aes_decrypt() {
        let decrypted = decipher_message(&[0x49, 0xf8, 0x34, 0x09, 0x0a, 0x8b, 0xce, 0xb2, 0x1c, 0x4f, 0xe3, 0x59, 0x64, 0x04, 0x52, 0xa9], "game of thrones!".as_bytes());
        assert_eq!(decrypted, Some("The Iron Throne.".to_string()));
    }
}
