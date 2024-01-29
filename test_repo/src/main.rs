use ethers::prelude::*;
use chrono::Utc;
use std::time::Duration;
use colored::Colorize;


const RPC_URL: &str = "https://eth.llamarpc.com";

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let mut old_block_number: U64 = provider.get_block_number().await?;
    loop {
        let new_block_number: U64 = provider.get_block_number().await?;
        if new_block_number != old_block_number{
            let current_time = Utc::now();
            println!("{current_time}\nCurrent Block Number: {new_block_number}");

            let block = provider.get_block_with_txs(BlockId::Number(new_block_number.into())).await;

            if let Ok(block_result) = block {
                if let Some(hash) = block_result.as_ref().and_then(|b| b.hash) {
                    println!("{}: {:?}", "Block Hash".red(), hash);
                }
                if let Some(timestamp) = block_result.as_ref().and_then(|b| Some(b.timestamp)) {
                    println!("{}: {:?}","Timestamp".red(), timestamp);
                }
                if let Some(gas_used) = block_result.as_ref().and_then(|b| Some(b.gas_used)) {
                    println!("{}: {:?}","Gas Used".red(), gas_used);
                }
                if let Some(gas_limit) = block_result.as_ref().and_then(|b| Some(b.gas_limit)) {
                    println!("{}: {:?}","Gas Limit".red(), gas_limit);
                }

                tokio::time::sleep(Duration::from_secs(3)).await;
                if let Some(transactions) = block_result.as_ref().and_then(|b| Some(b.transactions.iter())) {
                    for tx in transactions {
                        println!("{}: {:?}","Transaction Hash".green(), tx.hash);
                        println!("{}: {:?}","From".green(), tx.from);
                        println!("{}: {:?}","To".green(), tx.to);
                        println!("{}: {:?}","Value".green(), tx.value);
                    }
                }
            }
            else{
                println!("Failed to retrieve the block {:?}", block);
            }
            old_block_number = new_block_number;
            println!("");
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}