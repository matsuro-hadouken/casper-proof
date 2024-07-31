use casper_proof::cli;
use casper_proof::crypto::{derive_public_key_hex, sign_message, verify_signature};
use casper_proof::file_utils::{read_file_content, read_pem_file};
use casper_proof::key_info::{detect_key_type, print_key_info, KeyType};
use colored::Colorize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::build_cli().get_matches();

    if !matches.args_present() {
        cli::print_help();
        return Ok(());
    }

    let pub_key = matches.get_one::<String>("pub-key");
    let signature = matches.get_one::<String>("signature");
    let secret_key_pem_path = matches.get_one::<String>("pem-path");

    let message = if let Some(file_path) = matches.get_one::<String>("file") {
        read_file_content(file_path)?
    } else if let Some(text) = matches.get_one::<String>("text") {
        text.to_string()
    } else {
        "".to_string()
    };

    // Verification mode
    if let (Some(pub_key), Some(signature)) = (pub_key, signature) {
        if verify_signature(pub_key, &message, signature).is_ok() {
            println!("{}", "Signature is valid.".green());
        } else {
            println!("{}", "Signature is invalid.".red());
        }
    } else if let Some(secret_key_pem_path) = secret_key_pem_path {
        let secret_key_pem = read_pem_file(secret_key_pem_path)?;
        let key_info = detect_key_type(&secret_key_pem)?;
        let public_key_hex = derive_public_key_hex(&key_info)?;

        if matches.get_one::<String>("text").is_none() && matches.get_one::<String>("file").is_none() {
            let key_type = match key_info.key_type {
                KeyType::Ed25519 => "Ed25519",
                KeyType::Secp256k1 => "secp256k1",
            };
            print_key_info(secret_key_pem_path, key_type, &public_key_hex, secret_key_pem.len());
        } else if !message.is_empty() {
            let signature = sign_message(&key_info, &message)?;
            println!("{}", "===== Signature =====".bold().underline());
            println!("{}: {}", "Public Address".cyan(), public_key_hex.green());
            if matches.get_one::<String>("file").is_some() {
                println!("{}: {}", "File path".cyan(), matches.get_one::<String>("file").unwrap());
            } else {
                println!("{}: {}", "Message".cyan(), message);
            }
            println!("{}: {}", "Signature".cyan(), signature.green());
            println!("{}", "=====================".bold().underline());
        }
    } else {
        cli::print_help();
    }

    Ok(())
}
