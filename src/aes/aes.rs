//
// EPITECH PROJECT, 2024
// aes.rs
// File description:
// AES main function
//

use std::process::exit;

use crate::{aes::{cipher_message::cipher_message, decipher_message::decipher_message}, args_handling::args_struct::ArgsStruct};

use super::cipher_message::print_ciphered_message;

pub fn switch_endian(val: &Vec<u8>) -> Vec<u8> {
    val.chunks(4).flat_map(|word| word.iter().rev()).into_iter().map(|&c|c).collect()
}

fn aes_encrypt(args: ArgsStruct, display_result: bool) -> Option<String> {
    let little_endian_key = hex::decode(args.symmetric_key.unwrap().trim_end()).unwrap_or_else(|e| {eprintln!("{}", e); exit(84)});
    let cipher_key = switch_endian(&little_endian_key);
    let message = args.message.trim_end().as_bytes();

    if cipher_key.len() != message.len() && !args.stream_mode {
        eprintln!("Message should be the same size as the key.");
        return None;
    }
    let encrypted = cipher_message(message, cipher_key.as_slice());
    match &encrypted {
        Some(msg) => {
            if display_result {
                print_ciphered_message(&msg, args.stream_mode);
            }
            Some(String::from_utf8_lossy(encrypted.unwrap().as_slice()).into_owned())
        },
        None => {
            eprintln!("Couldn't encrypt message.");
            None
        }
    }
}

fn aes_decrypt(args: ArgsStruct, display_result: bool) -> Option<String> {
    let little_endian_key = hex::decode(args.symmetric_key.unwrap().trim_end()).unwrap_or_else(|e| {eprintln!("{}", e); exit(84)});
    let cipher_key = switch_endian(&little_endian_key);
    let mut message = hex::decode(args.message.trim_end()).unwrap_or_else(|e| {eprintln!("{}", e); exit(84)});
    message = switch_endian(&message);

    if cipher_key.len() != message.len() && !args.stream_mode {
        eprintln!("Message should be the same size as the key.");
        return None;
    }
    let decrypted = decipher_message(message.as_slice(), cipher_key.as_slice());
    match &decrypted {
        Some(msg) => {
            if display_result {
                if args.stream_mode {
                    print!("{msg}");
                } else {
                    println!("{msg}");
                }
            }
            Some(decrypted.unwrap())
        },
        None => {
            eprintln!("Couldn't decrypt message.");
            None
        }
    }
}

fn handle_aes(args: ArgsStruct, display_result: bool) -> Option<String> {
    if !args.cipherd_to_decipherd {
        aes_encrypt(args, display_result)
    } else {
        aes_decrypt(args, display_result)
    }
}

pub fn aes(args: ArgsStruct, display_result: bool) -> Option<String> {
    if args.stream_mode {
        let messages: Vec<String> = args.message.as_bytes().chunks(if args.cipherd_to_decipherd {32} else {16}).map(|msg| {
            let mut tmp = msg.to_vec();
            if !args.cipherd_to_decipherd {
                tmp.resize(16, 0);
            }
            String::from_utf8_lossy(tmp.as_slice()).into_owned()
        }).collect();
        let mut res = String::new();
        for msg in messages {
            let mut new_args = args.clone();
            new_args.message = msg;
            res += handle_aes(new_args, display_result)?.as_str();
        }
        println!();
        Some(res)
    } else {
        handle_aes(args, display_result)
    }
}

#[cfg(test)]
mod tests {
    use super::switch_endian;

    #[test]
    fn test_switch_endian() {
        let little_endian = hex::decode("57696e74657220697320636f6d696e67").unwrap();
        let big_endian = switch_endian(&little_endian);
        assert_eq!(big_endian, vec![0x74, 0x6e, 0x69, 0x57, 0x69, 0x20, 0x72, 0x65, 0x6f, 0x63, 0x20, 0x73, 0x67, 0x6e, 0x69, 0x6d]);
    }
}
