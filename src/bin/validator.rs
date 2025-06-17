// use std::fs;
use tonic::{Request, Response, Status, transport::Server};

use lunaria::{account::Address, ledger::Ledger};

use validator::validator_server::{Validator, ValidatorServer};
use validator::{BalanceReply, BalanceRequest};

pub mod validator {
    tonic::include_proto!("validator");
}

#[derive(Debug)]
pub struct MyValidator {
    ledger: Ledger,
}

#[tonic::async_trait]
impl Validator for MyValidator {
    async fn get_balance(
        &self,
        request: Request<BalanceRequest>,
    ) -> Result<Response<BalanceReply>, Status> {
        println!("Got a request: {:?}", request);

        let request_message = request.get_ref().clone();
        let balance = match Address::try_from(request_message.address.as_str()) {
            Ok(address) => self.ledger.balance(address),
            Err(e) => {
                eprintln!("invalid address");
                return Err(Status::internal(format!("invalid address: {e:?}")));
            }
        };

        let reply = BalanceReply {
            address: request_message.address,
            balance,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let ledger = Ledger::new()?;
    let validator = MyValidator { ledger };

    Server::builder()
        .add_service(ValidatorServer::new(validator))
        .serve(addr)
        .await?;

    Ok(())
}
