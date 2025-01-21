//
// EPITECH PROJECT, 2024
// main.rs
// File description:
// my_pgp main function
//

use std::{env, process::exit};

mod message_handling;
mod args_handling;
mod xor;
mod rsa;
mod aes;
mod pgp_aes;
mod pgp_xor;

fn cipherd_message(args: args_handling::args_struct::ArgsStruct) -> bool {
    let binding: String = args.crypto_system.clone();
    let system: &str = binding.as_str();

    let res = match system {
        "xor" => xor::xor::xor(args, true),
        "aes" => aes::aes::aes(args, true),
        "rsa" => rsa::rsa::rsa(args, true),
        "pgp-xor" => pgp_xor::pgp_xor::pgp_xor(args),
        "pgp-aes" => pgp_aes::pgp_aes::pgp_aes(args),
        _ => {
            eprintln!("Unknown encryption type.");
            None
        }
    };
    res.is_some()
}

fn print_usage() {
    eprintln!("USAGE
./my_pgp CRYPTO_SYSTEM MODE [OPTIONS] [key]

DESCRIPTION
\tThe MESSAGE is read from standard input
CRYPTO_SYSTEM
\t\"xor\" computation using XOR algorithm
\t\"aes\" computation using 128-bit AES algorithm
\t\"rsa\" computation using RSA algorithm
\t\"pgp-xor\" computation using both RSA and XOR algorithm
\t\"pgp-aes\" computation using both RSA and 128-bit AES algorithm
MODE
\t-c MESSAGE is clear and we want to cipher it
\t-d MESSAGE is ciphered and we want to decipher it
\t-g P Q for RSA only: Don't read a MESSAGE, but instead generate a public and
\tprivate key pair from the prime number P and Q
\tOPTIONS
\t-b for XOR, AES and PGP, only works on one block. The MESSAGE and the
\tsymmetric key must be the same size
\tkey Key used to cipher/decipher MESSAGE (incompatible with -g MODE)")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_min_size: usize = 3;

    if args.len() < args_min_size {
        print_usage();
        exit(84);
    }

    let tmp = args_handling::args_struct::parse_args(args);
    if tmp.is_none() {
        exit(84);
    }

    let mut stored_argument = tmp.unwrap();
    if stored_argument.asymmetric_key.is_none() && stored_argument.symmetric_key.is_none() {
        exit(84);
    }
    match stored_argument.asymmetric_key {
        Some(_) => {
            if cipherd_message(stored_argument) == false {
                exit(84);
            }
        },
        None => {
            let message: Result<String, std::io::Error> = message_handling::message_handling::get_message();

            match message {
                Ok(message) => stored_argument.message = message,
                Err(_) => {
                    eprintln!("No message read.");
                    exit(84);
                }
            }
            if cipherd_message(stored_argument) == false {
                exit(84);
            }
        }
    }
}
