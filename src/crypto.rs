use ed25519_dalek::{SigningKey, Signer, VerifyingKey, Signature, Verifier}; // Import Verifier trait
use secp256k1::{Secp256k1, SecretKey, PublicKey, Message};
use sha2::{Sha256, Digest};
use hex;

use crate::key_info::{KeyInfo, KeyType};

pub fn derive_public_key_hex(key_info: &KeyInfo) -> Result<String, Box<dyn std::error::Error>> {
    match key_info.key_type {
        KeyType::Ed25519 => {
            let signing_key = SigningKey::from_bytes(&key_info.private_key[..32].try_into()?);
            let public_key = VerifyingKey::from(&signing_key);
            let mut full_key = vec![1u8]; // 01 prefix for ED25519
            full_key.extend_from_slice(public_key.as_bytes());
            Ok(hex::encode(full_key))
        }
        KeyType::Secp256k1 => {
            let secp = Secp256k1::new();
            let secret_key = SecretKey::from_slice(&key_info.private_key)?;
            let public_key = PublicKey::from_secret_key(&secp, &secret_key);
            let mut full_key = vec![2u8]; // 02 prefix for secp256k1
            full_key.extend_from_slice(&public_key.serialize()[..]);
            Ok(hex::encode(full_key))
        }
    }
}

pub fn sign_message(key_info: &KeyInfo, message: &str) -> Result<String, Box<dyn std::error::Error>> {
    match key_info.key_type {
        KeyType::Ed25519 => {
            let signing_key = SigningKey::from_bytes(&key_info.private_key[..32].try_into()?);
            let signature = signing_key.sign(message.as_bytes());
            Ok(hex::encode(signature.to_bytes()))
        }
        KeyType::Secp256k1 => {
            let secp = Secp256k1::new();
            let secret_key = SecretKey::from_slice(&key_info.private_key)?;
            let mut hasher = Sha256::new();
            hasher.update(message.as_bytes());
            let message_hash = hasher.finalize();
            let message = Message::from_digest_slice(&message_hash)?;
            let signature = secp.sign_ecdsa(&message, &secret_key);
            Ok(hex::encode(signature.serialize_compact()))
        }
    }
}

pub fn verify_signature(pub_key: &str, message: &str, signature: &str) -> Result<(), Box<dyn std::error::Error>> {
    let key_type = match &pub_key[0..2] {
        "01" => KeyType::Ed25519,
        "02" => KeyType::Secp256k1,
        _ => return Err("Unknown key type".into()),
    };

    let pub_key_bytes = hex::decode(&pub_key[2..])?;
    let signature_bytes = hex::decode(signature)?;

    match key_type {
        KeyType::Ed25519 => {
            let public_key = VerifyingKey::from_bytes(&pub_key_bytes.try_into().map_err(|_| "Invalid public key length")?)?;
            let signature = Signature::from_bytes(&signature_bytes.try_into().map_err(|_| "Invalid signature length")?);
            public_key.verify(message.as_bytes(), &signature).map_err(|_| "Invalid signature")?;
        }
        KeyType::Secp256k1 => {
            let secp = Secp256k1::new();
            let public_key = PublicKey::from_slice(&pub_key_bytes)?;
            let mut hasher = Sha256::new();
            hasher.update(message.as_bytes());
            let message_hash = hasher.finalize();
            let message = Message::from_digest_slice(&message_hash)?;
            let signature = secp256k1::ecdsa::Signature::from_compact(&signature_bytes)?;
            secp.verify_ecdsa(&message, &signature, &public_key)?;
        }
    }
    Ok(())
}
