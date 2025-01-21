//
// EPITECH PROJECT, 2024
// cipher_message.rs
// File description:
// AES ciphering
//

use super::{add_round_key::add_round_key, key_expansion::key_expansion, matrix::Matrix, mix_columns::mix_columns, shift_rows::shift_rows, sub_bytes::sub_bytes, State};

fn encrypt_block(mut state: &mut State, keys: &State) -> Option<bool> {
    let nb_rounds = keys.col_size / 4;

    add_round_key(&mut state, &keys, 0);
    for round in 1..nb_rounds {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        if round != (nb_rounds - 1) {
            mix_columns(&mut state);
        }
        add_round_key(&mut state, &keys, round);
    }
    Some(true)
}

pub fn cipher_message(message: &[u8], cipher_key: &[u8]) -> Option<Vec<u8>> {
    let keys = key_expansion(cipher_key)?;
    let mut ciphered_text: Vec<u8> = vec![];

    for chunk in message.chunks(16) {
        let mut state = Matrix::from_key(chunk, (4, 4));
        encrypt_block(&mut state, &keys)?;
        ciphered_text.extend(state.flatten_cols());
    }
    Some(ciphered_text)
}

pub fn print_ciphered_message(ciphered_text: &Vec<u8>, stream_mode: bool) {
    let words: Vec<u8> = ciphered_text.chunks(4).flat_map(|word| word.iter().rev()).into_iter().map(|&c| c).collect();
    if stream_mode {
        print!("{}", hex::encode(words));
    } else {
        println!("{}", hex::encode(words));
    }
}

#[cfg(test)]
mod tests {
    use super::cipher_message;

    #[test]
    fn test_aes_encrypt_bootstrap() {
        let encrypted = cipher_message("The Iron Throne.".as_bytes(), "game of thrones\n".as_bytes());
        assert_eq!(encrypted, Some(vec![0x9c, 0x10, 0x9c, 0x7d, 0xe1, 0x8f, 0x08, 0x88, 0x4a, 0xf1, 0xb6, 0xdd, 0x57, 0x5a, 0x04, 0x02]));
    }

    #[test]
    fn test_aes_encrypt_subject() {
        let encrypted = cipher_message("All men must die".as_bytes(), &[0x74, 0x6e, 0x69, 0x57, 0x69, 0x20, 0x72, 0x65, 0x6f, 0x63, 0x20, 0x73, 0x67, 0x6e, 0x69, 0x6d]);
        assert_eq!(encrypted, Some(vec![0x2c, 0xe2, 0x4c, 0x74, 0x34, 0x58, 0x59, 0x38, 0x6e, 0xf2, 0x0d, 0x8f, 0xef, 0x2e, 0xb6, 0xce]));
    }
}
