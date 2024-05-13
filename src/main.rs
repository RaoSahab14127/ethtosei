use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use std::sync::Arc;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contract_address = "0x7B957DE1E68cdc4F223CaD2d81e46C24f486bAf7".parse::<Address>()?;
    abigen!(IERC721, "./src/abi.json");
    let rpc_url = "https://sepolia.infura.io/v3/96d3ce009f844c15ae18ea86722aa2b9";
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(provider);
    let contract = IERC721::new(contract_address, provider.clone());

    let function_name= "getOwner";
    let function_params = ();
    let result: Address = contract.method(function_name, function_params)?.call().await?;
    println!("{}", result);
    Ok(())
}