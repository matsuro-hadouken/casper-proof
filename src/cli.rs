use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("casper-proof")
        .about("Utility to derive public key from Casper secret key PEM files, sign messages, and verify signatures")
        .long_about(
            "Utility to derive public key from Casper secret key PEM files, sign messages, and verify signatures\n\n\
             Usage Examples:\n\n\
             # Providing only private key path will print PEM overview\n\
             casper-proof --pem-path path/to/secret_key.pem\n\n\
             # Sign text message, use single quotation\n\
             casper-proof --pem-path path/to/secret_key.pem --text '<TEXT>'\n\n\
             # Validate signature, use single quotation\n\
             casper-proof --pub-key <PUBLIC HEX ADDRESS> --signature <SIGNATURE> --text '<TEXT>'\n\n\
             # Sign message written in file\n\
             casper-proof --pem-path /path/to/secret_key.pem --file /path/to/foo.bar\n\n\
             # Verify message in file\n\
             casper-proof --pub-key <PUBLIC HEX ADDRESS> --file /path/to/foo.bar --signature <SIGNATURE>\n"
        )
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
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .help("Print help")
                .action(clap::ArgAction::Help)
        )
}

pub fn print_help() {
    build_cli().print_help().unwrap();
}
