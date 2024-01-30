pub mod web3 {
    use std::env;
    use std::str::FromStr;
    use web3::types::{Address, BlockId, BlockNumber, H160, U256, U64};

    fn wei_to_eth(wei_val: U256) -> f64 {
        let res = wei_val.as_u128() as f64;
        res / 1_000_000_000_000_000_000.0
    }

    #[tokio::main]
    pub async fn main() -> web3::Result<()> {
        dotenv::dotenv().ok();

        // Build the connection to the network
        let websocket =
            web3::transports::WebSocket::new(&env::var("INFURA_SEPOLIA").unwrap()).await?;
        let web3s = web3::Web3::new(websocket);

        // Get accounts from the connected node
        let mut accounts = web3s.eth().accounts().await?;
        accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
        println!("Accounts: {:?}", accounts);

        // Print those accounts' balances converted to Eth
        for account in accounts {
            let balance = web3s.eth().balance(account, None).await?;
            println!("Eth balance of {:?} {}", account, wei_to_eth(balance));
        }

        let mut previous_block_number: U64 = U64([u64::min_value(); 1]);
        while true {
            // Get the latest block
            let latest_block = web3s
                .eth()
                .block(BlockId::Number(BlockNumber::Latest))
                .await
                .unwrap()
                .unwrap();

            let blockNumber = latest_block.number.unwrap();

            // Do not print block if that one was already printed
            if blockNumber != previous_block_number {
                println!(
                    "block number {}, number of transactions: {}, difficulty {}",
                    latest_block.number.unwrap(),
                    &latest_block.transactions.len(),
                    &latest_block.total_difficulty.unwrap()
                );
            }

            previous_block_number = blockNumber;
        }
        Ok(())
    }
}

pub mod my_module {
    pub fn greeting() -> String {
        "Hello World!".to_string()
    }
}

fn main() {
    let message = my_module::greeting();
    println!("{}", message);
    let _ = web3::main();
}

#[test]
fn test_greeting() {
    let result = my_module::greeting();
    assert_eq!(result, "Hello World!");
}
