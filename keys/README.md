This repository contains a set of cryptographic keys in different formats _( Ed25519 and secp256k1 )_ for testing and experimentation purposes. These keys are pre-generated, allowing you to experiment with cryptographic operations and blockchain technologies without needing to generate your own keys via `casper-client`.

#### Ed25519
- **public_key_hex**: The public key represented in hexadecimal
- **public_key.pem**: The public key in PEM format
- **secret_key.pem**: The private key

#### secp256k1
- **public_key_hex**: The public key represented in hexadecimal
- **public_key.pem**: The public key in PEM format
- **secret_key.pem**: The private key

### Key Prefixes

To distinguish between the key types, the following prefixes are used in their hexadecimal representation:

- **01**: Indicates an Ed25519 public key. This prefix helps identify keys based on the Ed25519 algorithm.
- **02**: Indicates a secp256k1 public key. Widely used in blockchain applications, notably in Bitcoin.

### Generating Your Own Keys

For those who wish to generate their own keys, the following commands can be used with `casper-client`:

```sh
# Generate Ed25519 keys (public key prefix: 01)
casper-client keygen --algorithm Ed25519
```

```sh
# Generate secp256k1 keys (public key prefix: 02)
casper-client keygen --algorithm secp256k1
```