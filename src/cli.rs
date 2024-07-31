use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("casper-keys")
        .about("Utility to derive public key from Casper secret key PEM files, sign messages, and verify signatures")
        .arg(
            Arg::new("pem-path")
                .long("pem-path")
                .value_name("FILE")
                .help("Specifies the path to the secret key PEM file")
                .required(false)
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("text")
                .long("text")
                .value_name("TEXT")
                .help("The text message to sign or verify")
                .required(false)
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("file")
                .long("file")
                .value_name("FILE")
                .help("The path to a file whose content will be signed or verified")
                .required(false)
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("pub-key")
                .long("pub-key")
                .value_name("HEX")
                .help("The public key for verification in hex format")
                .required(false)
                .action(clap::ArgAction::Set),
        )
        .arg(
            Arg::new("signature")
                .long("signature")
                .value_name("HEX")
                .help("The signature to verify in hex format")
                .required(false)
                .action(clap::ArgAction::Set),
        )
}

pub fn print_help() {
    build_cli().print_help().unwrap();
}
