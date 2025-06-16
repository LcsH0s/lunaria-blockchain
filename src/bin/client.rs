use lunaria::client::{Client, DEFAULT_CREDS_LOCATION};

use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Generate,
    Account,
}

fn generate() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.save()?;
    Ok(())
}

fn account() -> Result<(), Box<dyn std::error::Error>> {
    match Client::from_default_path() {
        Ok(client) => {
            let (pk, _) = client.keypair();
            let address = client.address();
            println!("PublicKey: {}", hex::encode(pk));
            println!("SecretKey: ********");
            println!("Address: {address}");

            Ok(())
        }
        Err(_) => {
            println!("Unable to open wallet at {DEFAULT_CREDS_LOCATION}");
            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Generate {}) => generate(),
        Some(Commands::Account {}) => account(),
        None => {
            Cli::command().print_help()?;
            Ok(())
        }
    }
}
