# My pgp

the aim of the “my_pgp” project is to recreate five cryptographic models:
- aes
- rsa
- xor
- pgp_aes
- pgp_xor

To make this project we decided to do it in rust for its security of bigint management.

## Usage

### To build the project:
```sh
    make
```


```sh
./my_pgp CRYPTO_SYSTEM MODE [OPTIONS] [key]
DESCRIPTION
    The MESSAGE is read from standard input
CRYPTO_SYSTEM
    \"xor\" computation using XOR algorithm
    \"aes\" computation using 128-bit AES algorithm
    \"rsa\" computation using RSA algorithm
    \"pgp-xor\" computation using both RSA and XOR algorithm
    \"pgp-aes\" computation using both RSA and 128-bit AES algorithm
MODE
    -c MESSAGE is clear and we want to cipher it
    -d MESSAGE is ciphered and we want to decipher it
    -g P Q for RSA only: Don't read a MESSAGE, but instead generate a public and
    private key pair from the prime number P and Q
    OPTIONS
    -b for XOR, AES and PGP, only works on one block. The MESSAGE and the
    symmetric key must be the same size
    key Key used to cipher/decipher MESSAGE (incompatible with -g MODE)
```

## Co-Contributors

<table>
  <tr>
    <td align="center">
      <a href="https://github.com/Aluxray">
        <img src="https://avatars.githubusercontent.com/u/100275038?v=4" width=85><br>
        <sub>Alexandre Vigoureux</sub>
      </a>
    </td>
    <td align="center">
      <a href="https://github.com/edward-lalande">
        <img src="https://avatars.githubusercontent.com/u/114470214?v=4" width=85><br>
        <sub>Edward Lalande</sub>
      </a>
    </td>
  </tr>
</table>
