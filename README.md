# casper-proof console tool

**To build:** `cargo build --release`

### Disclaimer

**This project is a proof of concept and has not undergone a professional audit. It was created for personal cryptographic exploration and learning. As someone not professionally trained in cryptography and still gaining experience in Rust, I developed this tool to understand how Casper generates keys and whether they can be utilized for signing messages. If you have the expertise to audit this code, your feedback would be greatly appreciated. While this tool is primarily educational, it has the potential for broader applications, and ensuring its security and reliability is essential.**

A utility to derive public key from [Casper](https://github.com/casper-network/casper-node) secret key PEM files, sign messages, and verify signatures.

This is `console-client` tool which provide multiple functionalities and accept both `Ed25519` and `secp256k1`

* Get data from private key
* Sign text message
* Sign message in file
* Verify signature

#### Options:

      --pem-path <FILE>  Specifies the path to the secret key PEM file
      --text <TEXT>      The text message to sign or verify
      --file <FILE>      The path to a file whose content will be signed or verified ( accept valid UTF-8 format only )
      --pub-key <HEX>    The public key for verification in hex format
      --signature <HEX>  The signature to verify in hex format
      --help             Print help

#### Examples:

```bash
# Providing only private key path will print some information about this key
./casper-proof --pem-path keys/secp256k1/secret_key.pem
```
```
===== Casper Key Information =====
Using: keys/secp256k1/secret_key.pem
Key Type: secp256k1
Secret Key (PEM) Length: 48
Public Address: 0203b ... a3fc
===================================
```
```bash
# Sign text message
./casper-proof --pem-path keys/secp256k1/secret_key.pem --text 'This address is belong to me'
```
```
===== Signature =====
Public Address: 0203b ... a3fc
Message: This address is belong to me
Signature: 4bd4a ... 0e73
=====================
```
```bash
# Validate signature
./casper-proof --pub-key 0203...a3fc --signature 4bd4...0e73 --text 'This address is belong to me'
```
```
Signature is valid.
```
```bash
# sing message written in file ( accept valid UTF-8 format only )
./casper-proof --pem-path keys/secp256k1/secret_key.pem --file file.txt
```
```
===== Signature =====
Public Address: 0203b...a3fc
File path: file.txt
Signature: 6440...13d4
=====================
```
```bash
# verify message in file
./casper-proof --pub-key 0203...a3fc --file file.txt --signature 6440...13d4
```
```
Signature is valid.
```