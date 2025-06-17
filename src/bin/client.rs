use lunaria::client::{Client, DEFAULT_CREDS_LOCATION};

use validator::BalanceRequest;
use validator::validator_client::ValidatorClient;

pub mod validator {
    tonic::include_proto!("validator");
}

use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate a new public/private key pair and derives the address", long_about = None)]
    Generate,
    #[command(about = "Display account information", long_about = None)]
    Account,
    #[command(about = "Query balance of current account", long_about = None)]
    Balance,
}

async fn generate() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client.save()?;
    Ok(())
}

async fn account() -> Result<(), Box<dyn std::error::Error>> {
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

async fn get_balance() -> Result<(), Box<dyn std::error::Error>> {
    let mut grpc_client = ValidatorClient::connect("http://[::1]:50051").await?;

    match Client::from_default_path() {
        Ok(client) => {
            let address = client.address().to_string();
            let request = tonic::Request::new(BalanceRequest { address });

            let response = grpc_client.get_balance(request).await?;
            let response_msg = response.get_ref().clone();

            println!("{} LUN", response_msg.balance);

            Ok(())
        }
        Err(_) => Ok(()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Generate {}) => generate().await,
        Some(Commands::Account {}) => account().await,
        Some(Commands::Balance {}) => get_balance().await,
        None => {
            Cli::command().print_help()?;
            Ok(())
        }
    }
}
