use casper_proof::file_utils::{read_pem_file};
use casper_proof::key_info::{detect_key_type, KeyType, print_key_info};
use casper_proof::crypto::{sign_message, derive_public_key_hex, verify_signature};
use colored::*;
use base64::{engine::general_purpose, Engine as _};
use std::error::Error;
use std::fs::File;
use std::fmt;
use std::io::{BufReader, Read};

// Maximum allowed file size (1 MB)
const MAX_FILE_SIZE: u64 = 1 * 1024 * 1024;

// Custom error type for file validation errors
#[derive(Debug)]
struct FileValidationError {
    details: String,
}

impl FileValidationError {
    fn new(msg: &str) -> FileValidationError {
        FileValidationError{details: msg.to_string()}
    }
}

impl fmt::Display for FileValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for FileValidationError {}

// Function to validate file content
pub fn validate_file_content(file_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);

    // Check for file size limit by reading in chunks and counting the total
    let mut content = Vec::new();
    let mut total_size = 0;
    let mut buffer = [0; 1024]; // Buffer to read data in chunks
    loop {
        let bytes_read = buf_reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        total_size += bytes_read as u64;
        if total_size > MAX_FILE_SIZE {
            return Err(Box::new(FileValidationError::new(
                &format!("File size exceeds the maximum allowed limit of {} bytes", MAX_FILE_SIZE)
            )));
        }
        content.extend_from_slice(&buffer[..bytes_read]);
    }

    if content.is_empty() {
        return Err(Box::new(FileValidationError::new("File is empty")));
    }

    Ok(content)
}

// Main function for CLI processing and logic
fn main() -> Result<(), Box<dyn Error>> {
    let matches = casper_proof::cli::build_cli().get_matches();

    if !matches.args_present() {
        casper_proof::cli::build_cli().print_help()?;
        println!();
        return Ok(());
    }

    // Determine the message to be signed or verified
    let message = if let Some(file_path) = matches.get_one::<String>("file") {
        validate_file_content(file_path)? // Validate the file content before processing
    } else if let Some(text) = matches.get_one::<String>("text") {
        text.as_bytes().to_vec()
    } else {
        Vec::new()
    };

    // Base64 encode the message for signing and verification
    let message_base64 = general_purpose::STANDARD.encode(&message);

    // Check for public key and signature for verification
    if let (Some(pub_key), Some(signature)) = (
        matches.get_one::<String>("pub-key"),
        matches.get_one::<String>("signature"),
    ) {
        match verify_signature(pub_key, &message_base64, signature) {
            Ok(_) => println!("{}", "Signature is valid.".bold().green()),
            Err(e) => println!("{}: {}", "Signature verification failed".bold().red(), e),
        }
        return Ok(());
    }

    // Handling the PEM path for signing
    if let Some(pem_path) = matches.get_one::<String>("pem-path") {
        let secret_key_pem = read_pem_file(pem_path)?;
        let key_info = detect_key_type(&secret_key_pem)?;
        let public_key_hex = derive_public_key_hex(&key_info)?;

        if message.is_empty() {
            let key_type = match key_info.key_type {
                KeyType::Ed25519 => "Ed25519",
                KeyType::Secp256k1 => "Secp256k1",
            };
            print_key_info(pem_path, key_type, &public_key_hex, secret_key_pem.len());
            return Ok(());
        }

        // Sign the message
        let signature = sign_message(&key_info, &message_base64)?;

        // Output the results
        println!("{}", "===== Signature =====".bold().underline());
        println!("{}: {}", "Public Address".cyan(), public_key_hex.green());
        if let Some(file_path) = matches.get_one::<String>("file") {
            println!("{}: {}", "File path".cyan(), file_path);
        } else {
            println!("{}: {}", "Message".cyan(), "Binary data");
        }
        println!("{}: {}", "Signature".cyan(), signature.green());
        println!("{}", "=====================".bold().underline());
    } else {
        eprintln!("No PEM file path provided for signing. Unable to sign the message.");
    }

    Ok(())
}
