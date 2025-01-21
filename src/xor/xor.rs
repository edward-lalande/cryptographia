//
// EPITECH PROJECT, 2024
// B-CNA-500-PAR-5-1-cryptography-edward.lalande
// File description:
// xor
//

use std::process::exit;

use hex;
use num_bigint::BigUint;
use crate::args_handling::args_struct::ArgsStruct;

fn str_to_biguint_from_hex(key: &str) -> BigUint {
    BigUint::from_bytes_le(&hex::decode(key).unwrap_or_else(|e| {eprintln!("str_to_biguint_from_hex(): {}", e); exit(84)}))
}

fn biguint_xor(big_message: &BigUint, big_key: &BigUint) -> BigUint {
    big_message ^ big_key
}

fn encrypt_in_xor(key: &str, message: String) -> Option<String> {
    let big_key = str_to_biguint_from_hex(key);
    let message_len = hex::encode(&message).len();

    if key.len() != message_len {
        eprintln!("Key len is different of the message len");
        return None;
    }

    let encrypted = message.into_bytes()
                                                .iter()
                                                .zip(big_key
                                                       .to_bytes_be()
                                                       .iter()
                                                       .cycle())
                                                       .map(|(c,c_x)|
                                                           c ^ c_x).collect::<Vec<u8>>();

    Some(format!("{:x}", BigUint::from_bytes_le(encrypted.as_slice())))
}

fn decrypt_in_xor(key: &str, message: &str) -> Option<String> {
    let encrypted_bytes = hex::decode(message).unwrap_or_else(|e| {eprintln!("decrypt_in_xor() -> hex::decode(): {}", e); exit(84)});
    let big_message = BigUint::from_bytes_le(&encrypted_bytes);
    let big_key = str_to_biguint_from_hex(key);
    let decrypted = biguint_xor(&big_message, &big_key);

    if key.len() != message.len() {
        eprintln!("Key len is different of the message len");
        return None;
    }

    Some(String::from_utf8(decrypted.to_bytes_le()).unwrap_or_else(|e| {eprintln!("decrypt_in_xor() -> String::from_utf8(): {}", e); exit(84)}))
}

pub fn xor(args: ArgsStruct, display_result: bool) -> Option<String> {
    let key: String = args.symmetric_key.unwrap();
    let message: String = args.message;

    if !args.cipherd_to_decipherd {
        let res: String = encrypt_in_xor(&key, message)?;

        if display_result {
            println!("{}", res.clone());
        }

        Some(res)
    } else {
        let decrypted: String = decrypt_in_xor(&key, &message)?;

        let mut decrypted_message: Vec<u8> = decrypted.as_bytes().to_owned();
        decrypted_message.reverse();
        let res: String = String::from_utf8(decrypted_message).unwrap();

        if display_result {
            println!("{}", res);
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{args_handling::args_struct::ArgsStruct, xor::xor::{decrypt_in_xor, encrypt_in_xor, xor}};

    #[test]
    fn test_good_xor_decipher() {
        let key = "576861742069732064656164206d6179206e6576657220646965";
        let message = "20070f2700071c6a4449060a490515164e4e12190b190011063c";

        assert_eq!(xor(ArgsStruct { crypto_system: "xor".to_string(),
                                    cipherd_to_decipherd: true,
                                    asymmetric_key: None,
                                    symmetric_key: Some(key.to_string()),
                                    stream_mode: true,
                                    message: message.to_string() },
                                    true
                      ),
            Some("You know nothing, Jon Snow".to_string()));
    }

    #[test]
    fn test_good_xor_cipher() {
        let key = "576861742069732064656164206d6179206e6576657220646965";
        let message = "You know nothing, Jon Snow".to_string();

        assert_eq!(xor(ArgsStruct { crypto_system: "xor".to_string(),
                                    cipherd_to_decipherd: false,
                                    asymmetric_key: None,
                                    symmetric_key: Some(key.to_string()),
                                    stream_mode: true,
                                    message: message },
                                    true
                      ),
            Some("20070f2700071c6a4449060a490515164e4e12190b190011063c".to_string()));
    }

    #[test]
    fn test_bad_xor_encryption() {
        let wrong_size_key = "42";
        let message = "You know nothing, Jon Snow".to_string();

        assert_eq!(xor(ArgsStruct { crypto_system: "xor".to_string(),
                                    cipherd_to_decipherd: false,
                                    asymmetric_key: None,
                                    symmetric_key: Some(hex::encode(wrong_size_key)),
                                    stream_mode: true,
                                    message: message },
                                    true
                      ),
            None);
    }

    #[test]
    fn test_xor_none() {
        let wrong_size_key = "42";
        let message = "You know nothing, Jon Snow".to_string();

        assert_eq!(encrypt_in_xor(wrong_size_key, message), None);
    }

    #[test]
    fn test_bad_block_size_decrypt_xor() {
        let wrong_size_key = "42";
        let message = "1234567890";

        assert_eq!(decrypt_in_xor(wrong_size_key, message), None);
    }
}
