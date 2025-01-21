//
// EPITECH PROJECT, 2024
// B-CNA-500-PAR-5-1-cryptography-edward.lalande
// File description:
// argsStrut
//

#[derive(Debug, Clone, PartialEq)]
pub struct ArgsStruct {
    pub crypto_system: String,
    pub cipherd_to_decipherd: bool,
    pub asymmetric_key: Option<(String, String)>,
    pub symmetric_key: Option<String>,
    pub stream_mode: bool,
    pub message: String,
}

pub fn get_crypto_system(system: &str) -> Option<String> {
    match system {
        "xor" => Some(system.to_string()),
        "aes" => Some(system.to_string()),
        "rsa" => Some(system.to_string()),
        "pgp-xor" => Some(system.to_string()),
        "pgp-aes" => Some(system.to_string()),
        _ => {
            eprintln!("Unknown crypto system type \"{system}\"");
            None
        }
    }
}

fn get_ciphered_to(args: &Vec<String>) -> Option<bool> {
    let generate: bool = args.clone().iter().find(|&str| str == "-g").is_some();
    let encrypt: bool = args.clone().iter().find(|&arg| arg == "-c").is_some();
    let decrypt: bool = args.clone().iter().find(|&arg| arg == "-d").is_some();

    if (generate && !encrypt && !decrypt) ||
        (!generate && encrypt && !decrypt) {
        Some(false)
    } else if !generate && !encrypt && decrypt {
        Some(true)
    } else {
        eprintln!("One and only one mode (\"-c\", \"-d\", \"-g\") must be given.");
        None
    }
}

fn get_symmetric_key(args: &Vec<String>) -> Option<String> {
    if args.iter().find(|&str| str == "-g").is_some() {
        return None;
    }
    if args.last()?.starts_with('-') {
        eprintln!("Last argument should be the key, not a flag.");
        None
    } else {
        args.last().cloned()
    }
}

fn get_asymmetric_key(args: &Vec<String>, crypto_system: &String) -> Option<(String, String)> {
    let args_size: usize = args.len();
    let cloned_args: Vec<String> = args.clone();

    for (idx, arg) in cloned_args.into_iter().enumerate() {
        if arg == "-g" && crypto_system == "rsa" {
            if idx + 2 < args_size {
                return Some((args[idx + 1].clone(), args[idx + 2].clone()))
            }
            eprintln!("Missing prime number P or Q for \"-g\" flag.");
            break;
        } else if arg == "-g" && crypto_system != "rsa" {
            eprintln!("Flag \"-g\" is only available for RSA encryption.");
            break;
        }
    }
    None
}

fn is_stream_mode(args: &Vec<String>) -> bool {
    args.iter().find(|&arg| arg == "-b").is_none()
}

fn is_valid_flag(flag: &str, crypto_system: &String) -> bool {
    match flag {
        "-c" => true,
        "-d" => true,
        "-g" => {
            if crypto_system == "rsa" {
                true
            } else {
                eprintln!("Mode \"-g\" is only available for RSA encryption.");
                false
            }
        },
        "-b" => {
            if crypto_system == "rsa" {
                eprintln!("Option \"-b\" is not available for RSA encryption.");
                false
            } else {
                true
            }
        },
        _ => {
            eprintln!("Unknown flag \"{flag}\".");
            false
        }
    }
}

fn args_valid(args: &Vec<String>, crypto_system: &String) -> bool {
    for arg in args {
        if arg.starts_with('-') && !is_valid_flag(arg.as_str(), crypto_system) {
            return false;
        }
    }
    true
}

pub fn parse_args(args: Vec<String>) -> Option<ArgsStruct> {
    let crypto_system = get_crypto_system(&args[1])?;
    if !args_valid(&args, &crypto_system) {
        return None;
    }
    return Some(ArgsStruct{
        crypto_system: crypto_system.clone(),
        cipherd_to_decipherd: get_ciphered_to(&args)?,
        asymmetric_key: get_asymmetric_key(&args, &crypto_system),
        symmetric_key: get_symmetric_key(&args),
        stream_mode: is_stream_mode(&args),
        message: "".to_string()
    });
}
