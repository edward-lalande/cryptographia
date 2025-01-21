use std::process::exit;

use hex;
use num_bigint::BigInt;
use num_traits::One;
use num_integer::Integer;
use crate::args_handling::args_struct::ArgsStruct;

fn str_to_bigint_from_hex(key: &str) -> BigInt {
    BigInt::from_bytes_le(num_bigint::Sign::Plus, &hex::decode(key).unwrap_or_else(|e| {eprintln!("str_to_bigint_from_hex(): {}", e); exit(84)}))
}

fn prime_number_from(phi_n: &BigInt) -> Option<BigInt> {
    let prime_vec: Vec<u32> = vec![3, 5, 17, 257, 65537];

    for e in prime_vec.iter().rev() {
        if BigInt::from(*e) < *phi_n {
            return Some(BigInt::from(*e));
        }
    }
    eprintln!("prime_number_from(): No prime number for {}", phi_n);
    None
}

fn encryption(message: String, key: String) -> String {
    let key_parts: Vec<&str> = key.split('-').collect();
    let e: BigInt = str_to_bigint_from_hex(key_parts[0]);
    let n: BigInt = str_to_bigint_from_hex(key_parts[1]);
    let m: BigInt = BigInt::from_bytes_le(num_bigint::Sign::Plus, message.as_bytes());
    let c: BigInt = m.modpow(&e, &n);

    hex::encode(c.to_bytes_le().1)
}

fn decryption(ciphertext: String, key: String) -> String {
    let key_parts: Vec<&str> = key.split('-').collect();
    let d: BigInt = str_to_bigint_from_hex(key_parts[0]);
    let n: BigInt = str_to_bigint_from_hex(key_parts[1]);
    let c: BigInt = BigInt::from_bytes_le(num_bigint::Sign::Plus, &hex::decode(ciphertext).unwrap_or_else(|e| {eprintln!("decryption() -> BigInt::from_bytes_le(): {}", e); exit(84)}));
    let m: BigInt = c.modpow(&d, &n);

    String::from_utf8(m.to_bytes_le().1).unwrap_or_else(|e| {eprintln!("decryption() -> String::from_utf8(): {}", e); exit(84)})
}

pub fn rsa(args: ArgsStruct, display_result: bool) -> Option<String> {
    let first_key: (String, String);

    if args.symmetric_key.is_none() {
        match args.asymmetric_key {
            Some(keys) => first_key = keys,
            None => {
                eprintln!("No cipher key found.");
                return None;
            }
        }

        let p: BigInt = str_to_bigint_from_hex(first_key.0.as_str());
        let q: BigInt = str_to_bigint_from_hex(first_key.1.as_str());
        let n: BigInt = &p * &q;
        let phi_n: BigInt = (&p - BigInt::one()).lcm(&(&q - BigInt::one()));
        let e: BigInt = prime_number_from(&phi_n)?;
        let d: BigInt = e.modinv(&phi_n).unwrap();

        if display_result {
            println!("public key: {}-{}", hex::encode(e.to_bytes_le().1), hex::encode(n.to_bytes_le().1));
            println!("private key: {}-{}", hex::encode(d.to_bytes_le().1), hex::encode(n.to_bytes_le().1));
        }
        return Some(format!("{}-{}\n{}-{}", hex::encode(e.to_bytes_le().1), hex::encode(n.to_bytes_le().1), hex::encode(d.to_bytes_le().1), hex::encode(n.to_bytes_le().1)));
    }

    let key: String = args.symmetric_key.unwrap();

    if !args.cipherd_to_decipherd {
        let encrypted = encryption(args.message, key);
        if display_result {
            println!("{encrypted}");
        }
        Some(encrypted)
    } else {
        let decrypted = decryption(args.message, key);
        if display_result {
            println!("{decrypted}");
        }
        Some(decrypted)
    }
}

#[cfg(test)]
mod test {
    use num_bigint::BigInt;

    use crate::{args_handling::args_struct::ArgsStruct, rsa::rsa::{decryption, encryption, prime_number_from}};

    use super::rsa;

    #[test]
    fn test_prime_number_from() {
        assert_eq!(prime_number_from(&BigInt::from(300)), Some(BigInt::from(257)));
    }

    #[test]
    fn test_rsa_encryption() {
        assert_eq!(encryption("WF".to_string(), "0101-19bb".to_string()), "8f84".to_string())
    }

    #[test]
    fn test_rsa_decryption() {
        assert_eq!(decryption("8f84".to_string(), "9d5b-19bb".to_string()), "WF".to_string());
    }

    #[test]
    fn test_easy_key_gen() {
        let message = "";
        assert_eq!(rsa(ArgsStruct { crypto_system: "xor".to_string(),
                                    cipherd_to_decipherd: true,
                                    asymmetric_key: Some(("d3".to_string(), "e3".to_string())),
                                    symmetric_key: None,
                                    stream_mode: true,
                                    message: message.to_string() },
                                    true
                      ),
            Some("0101-19bb\n9d5b-19bb".to_string()));
    }

    #[test]
    fn test_hard_key_gen() {
        let message = "";
        assert_eq!(rsa(ArgsStruct { crypto_system: "xor".to_string(),
                                    cipherd_to_decipherd: true,
                                    asymmetric_key: Some(("4b1da73924978f2e9c1f04170e46820d648edbee12ccf4d4462af89b080c86e1".to_string(), "bb3ca1e126f7c8751bd81bc8daa226494efb3d128f72ed9f6cacbe96e14166cb".to_string())),
                                    symmetric_key: None,
                                    stream_mode: true,
                                    message: message.to_string() },
                                    true
                      ),
            Some("010001-c9f91a9ff3bd6d84005b9cc8448296330bd23480f8cf8b36fd4edd0a8cd925de139a0076b962f4d57f50d6f9e64e7c41587784488f923dd60136c763fd602fb3\n81b08f4eb6dd8a4dd21728e5194dfc4e349829c9991c8b5e44b31e6ceee1e56a11d66ef23389be92ef7a4178470693f509c90b86d4a1e1831056ca0757f3e209-c9f91a9ff3bd6d84005b9cc8448296330bd23480f8cf8b36fd4edd0a8cd925de139a0076b962f4d57f50d6f9e64e7c41587784488f923dd60136c763fd602fb3".to_string()));
    }
}
