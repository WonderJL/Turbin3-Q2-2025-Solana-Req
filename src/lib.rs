use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    self,
    signature::{Keypair, Signer, read_keypair_file},
};
use std::io::{self, BufRead};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn get_key_pair() -> (String, [u8; 64]) {
    let kp = Keypair::new();
    let pubkey = kp.pubkey().to_string();
    let bytes_array = kp.to_bytes();
    (pubkey, bytes_array)
}

pub fn base58_to_byte_array(input: Option<String>) -> Vec<u8> {
    let base58 = match input {
        Some(key) => key,
        None => {
            println!("Input your private key as base58:");
            let stdin = io::stdin();
            stdin.lock().lines().next().unwrap().unwrap()
        }
    };

    let wallet = bs58::decode(&base58).into_vec().unwrap();
    println!("Your wallet file is:");
    println!("{:?}", wallet.clone());
    wallet
}

pub fn byte_array_to_base58(input: Option<Vec<u8>>) -> String {
    let wallet = match input {
        Some(bytes) => bytes,
        None => {
            println!("Input your private key as a wallet file byte array:");
            let stdin = io::stdin();
            stdin
                .lock()
                .lines()
                .next()
                .unwrap()
                .unwrap()
                .trim_start_matches('[')
                .trim_end_matches(']')
                .split(',')
                .map(|s| s.trim().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        }
    };

    let base58 = bs58::encode(&wallet).into_string();
    println!("Your private key is:");
    println!("{:?}", base58.clone());
    base58
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn keygen() {
        let (pubkey, bytes_array) = get_key_pair();
        // Create a new keypair
        println!("You've generated a new Solana wallet: {}", pubkey);
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", bytes_array);
    }

    #[test]
    fn convert() {
        // Generate a keypair
        let (_pubkey, bytes_array) = get_key_pair();

        // Convert bytes to base58
        let base58_str = byte_array_to_base58(Some(bytes_array.to_vec()));

        // Convert base58 back to bytes
        let recovered_bytes = base58_to_byte_array(Some(base58_str));

        // Verify the conversion is correct
        assert_eq!(recovered_bytes.as_slice(), bytes_array);
    }

    #[test]
    fn airdrop() {
        const RPC_URL: &str = "https://api.devnet.solana.com";

        // Import our keypair
        let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");

        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        }
    }

    #[test]
    fn transfer_sol() {}
}
