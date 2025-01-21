//
// EPITECH PROJECT, 2024
// B-CNA-500-PAR-5-1-cryptography-edward.lalande
// File description:
// message_handling
//

use std::io::{self, BufRead, Error, ErrorKind};

pub fn get_message() -> Result<String, std::io::Error> {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line1 = iterator.next().unwrap_or(Err(Error::new(ErrorKind::Other, "")));

    return line1;
}
