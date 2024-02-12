use std::io;
use std::thread;

pub mod s3 {
    // Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
    // SPDX-License-Identifier: Apache-2.0
    #![allow(clippy::result_large_err)]

    use aws_config::meta::region::RegionProviderChain;
    use aws_sdk_s3::primitives::ByteStream;
    use aws_sdk_s3::{config::Region, meta::PKG_VERSION, Client, Error};
    use clap::Parser;
    use std::path::Path;
    use std::process;
    use std::io;
    

    #[derive(Debug, Parser)]
    struct Opt {
        /// The AWS Region.
        #[structopt(short, long)]
        region: Option<String>,

        /// The name of the bucket.
        #[structopt(short, long)]
        bucket: String,

        /// The name of the file to upload.
        #[structopt(short, long)]
        filename: String,

        /// The name of the object in the bucket.
        #[structopt(short, long)]
        key: String,

        /// Whether to display additional information.
        #[structopt(short, long)]
        verbose: bool,
    }

    // Upload a file to a bucket.
    // snippet-start:[s3.rust.s3-helloworld]
    async fn upload_object(
        client: &Client,
        bucket: &str,
        filename: &str,
        key: &str,
    ) -> Result<(), Error> {
        let resp = client.list_buckets().send().await?;

        for bucket in resp.buckets() {
            println!("bucket: {:?}", bucket.name().unwrap_or_default())
        }

        println!();

        let body = ByteStream::from_path(Path::new(filename)).await;

        match body {
            Ok(b) => {
                let resp = client
                    .put_object()
                    .bucket(bucket)
                    .key(key)
                    .body(b)
                    .send()
                    .await?;

                println!("Upload success. Version: {:?}", resp.version_id);

                let resp = client.get_object().bucket(bucket).key(key).send().await?;
                let data = resp.body.collect().await;
                // println!("data: {:?}", data.unwrap().into_bytes());
            }
            Err(e) => {
                println!("Got an error uploading object:");
                println!("{}", e);
                process::exit(1);
            }
        }

        Ok(())
    }
    // snippet-end:[s3.rust.s3-helloworld]

    /// Lists your buckets and uploads a file to a bucket.
    /// # Arguments
    ///
    /// * `-b BUCKET` - The bucket to which the file is uploaded.
    /// * `-k KEY` - The name of the file to upload to the bucket.
    /// * `[-r REGION]` - The Region in which the client is created.
    ///    If not supplied, uses the value of the **AWS_REGION** environment variable.
    ///    If the environment variable is not set, defaults to **us-east-1**.
    /// * `[-v]` - Whether to display additional information.
    #[tokio::main]
    pub async fn main() -> Result<(), Error> {
        tracing_subscriber::fmt::init();

        println!("Enter bucket name:");
        let mut bucket = String::new();
        io::stdin().read_line(&mut bucket).expect("Failed to read line");
        let bucket = bucket.trim();

        println!("Enter filename:");
        let mut filename = String::new();
        io::stdin().read_line(&mut filename).expect("Failed to read line");
        let filename = filename.trim();

        println!("Enter key:");
        let mut key = String::new();
        io::stdin().read_line(&mut key).expect("Failed to read line");
        let key = key.trim();

        // let Opt {
        //     bucket,
        //     filename,
        //     key,
        //     region,
        //     verbose,
        // } = Opt::parse();

        // let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        //     .or_default_provider()
        //     .or_else(Region::new("us-east-1"));

        let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"));
        println!();

        // if verbose {
            println!("S3 client version: {}", PKG_VERSION);
            println!(
                "Region:            {}",
                region_provider.region().await.unwrap().as_ref()
            );
            println!("Bucket:            {}", &bucket);
            println!("Filename:          {}", &filename);
            println!("Key:               {}", &key);
            println!();
        // }

        let shared_config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&shared_config);

        upload_object(&client, &bucket, &filename, &key).await
    }
}

pub mod web3 {
    use chrono::{DateTime, Local, TimeZone};
    use std::collections::HashMap;
    use std::env;
    use std::str::FromStr;
    use std::thread;
    use std::time::Duration;
    use web3::types::{Address, BlockId, BlockNumber, H160, U256, U64};

    fn wei_to_eth(wei_val: U256) -> f64 {
        let res = wei_val.as_u128() as f64;
        res / 1_000_000_000_000_000_000.0
    }

    fn convert_date(timestamp_str: &str) -> DateTime<Local> {
        if let Ok(timestamp) = timestamp_str.parse::<i64>() {
            return Local.timestamp(timestamp, 0);
        } else {
            return Local.timestamp(0, 0);
        }
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

        // Used for caching latest block number
        let mut previous_block_number: U64 = U64([u64::min_value(); 1]);

        while true {
            // Get the latest block
            let latest_block = web3s
                .eth()
                .block(BlockId::Number(BlockNumber::Latest))
                .await
                .unwrap()
                .unwrap();

            let block_number = latest_block.number.unwrap();

            // Do not print block if that one was already printed
            if block_number > previous_block_number {
                println!(
                    "block number {}, number of transactions: {}, difficulty {} @ {}",
                    latest_block.number.unwrap(),
                    &latest_block.transactions.len(),
                    &latest_block.total_difficulty.unwrap(),
                    convert_date(&latest_block.timestamp.to_string())
                );
            }

            previous_block_number = block_number;

            // limits the number of requests we make
            thread::sleep(Duration::from_secs(1));
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
    println!("Point72 Blockchain Menu: \n(1) listener\n(2) Store Data in S3");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();

            if input == "1" {
                let _ = web3::main();
            } else if input == "2" {
                if let Err(e) = s3::main() {
                    eprintln!("Error running S3 main: {:?}", e);
                }
            } else {
                eprintln!("Error: choose 1 or 2 ");
            }
        }
        Err(error) => {
            // If an error occurred, print the error message
            eprintln!("Error reading input: {}", error);
        }
    }

    // println!("{}", message);
    // let _ = web3::main();
}

#[test]
fn test_greeting() {
    let result = my_module::greeting();
    assert_eq!(result, "Hello World!");
}
