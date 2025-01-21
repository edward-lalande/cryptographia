//
// EPITECH PROJECT, 2024
// B-CNA-500-PAR-5-1-cryptography-edward.lalande
// File description:
// pgp-xor
//

use crate::{args_handling::args_struct::{get_crypto_system, ArgsStruct}, rsa::rsa::rsa, xor::xor::xor};

pub fn pgp_xor(args: ArgsStruct) -> Option<String> {
    let tmp = args.symmetric_key.unwrap_or(String::new());
    let keys: Vec<&str> = tmp.split(":").collect();
    if keys.len() != 2 {
        eprintln!("pgp-xor requires a symmetric key and a RSA key formatted like so: \"sym_key:rsa_key\"");
        return None;
    }
    if !args.cipherd_to_decipherd {
        let mut res = rsa(ArgsStruct { crypto_system: get_crypto_system("rsa")?, cipherd_to_decipherd: args.cipherd_to_decipherd, asymmetric_key: None, symmetric_key: Some(keys[1].to_string()), stream_mode: args.stream_mode, message: String::from_utf8_lossy(hex::decode(keys[0]).unwrap().as_slice()).into_owned() }, true)?;
        res += "\n";
        res += xor(ArgsStruct { crypto_system: get_crypto_system("xor")?, cipherd_to_decipherd: args.cipherd_to_decipherd, asymmetric_key: None, symmetric_key: Some(keys[0].to_string()), stream_mode: args.stream_mode, message: args.message }, true)?.as_str();
        Some(res)
    } else {
        let cipher_key = rsa(ArgsStruct { crypto_system: get_crypto_system("rsa")?, cipherd_to_decipherd: args.cipherd_to_decipherd, asymmetric_key: None, symmetric_key: Some(keys[1].to_string()), stream_mode: args.stream_mode, message: keys[0].to_string() }, false)?;
        xor(ArgsStruct { crypto_system: get_crypto_system("xor")?, cipherd_to_decipherd: args.cipherd_to_decipherd, asymmetric_key: None, symmetric_key: Some(hex::encode(cipher_key)), stream_mode: args.stream_mode, message: args.message }, true)
    }
}
