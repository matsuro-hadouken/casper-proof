use colored::{Colorize};

pub enum KeyType {
    Ed25519,
    Secp256k1,
}

pub struct KeyInfo {
    pub key_type: KeyType,
    pub private_key: Vec<u8>,
}

pub fn detect_key_type(private_key_data: &[u8]) -> Result<KeyInfo, Box<dyn std::error::Error>> {
    if private_key_data.starts_with(&[0x30, 0x2e, 0x02, 0x01, 0x00, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x04, 0x22, 0x04, 0x20]) {
        Ok(KeyInfo {
            key_type: KeyType::Ed25519,
            private_key: private_key_data[16..48].to_vec(),
        })
    } else if private_key_data.starts_with(&[0x30, 0x2e, 0x02, 0x01, 0x01, 0x04, 0x20]) {
        Ok(KeyInfo {
            key_type: KeyType::Secp256k1,
            private_key: private_key_data[7..39].to_vec(),
        })
    } else {
        Err("Unknown key type".into())
    }
}

pub fn print_key_info(pem_path: &str, key_type: &str, public_address: &str, secret_key_pem_len: usize) {
    println!("{}", "===== Casper Key Information =====".bold().underline());
    println!("{}: {}", "Using".cyan(), pem_path);
    println!("{}: {}", "Key Type".cyan(), key_type);
    println!("{}: {}", "Secret Key (PEM) Length".cyan(), secret_key_pem_len);
    println!("{}: {}", "Public Address".green(), public_address);
    println!("{}", "===================================".bold().underline());
}
