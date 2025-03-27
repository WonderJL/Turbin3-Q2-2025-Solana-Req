use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::hash::hash;
use solana_sdk::{
    message::Message,
    signature::{Keypair, Signer, read_keypair_file},
    system_instruction,
    transaction::Transaction,
};
use std::io::{self, BufRead};
use std::str::FromStr;

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
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

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
    fn transfer_sol() {
        const RPC_URL: &str = "https://api.devnet.solana.com";

        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // With the imported Keypair, we can sign a new message
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());

        // Verify the signature using the default implementation
        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }

        // Define our Turbin3 public key
        let to_pubkey = Pubkey::from_str("GLtaTaYiTQrgz411iPJD79rsoee59HhEy18rtRdrhEUJ").unwrap();

        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn clear_wallet() {
        let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());
        
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("GLtaTaYiTQrgz411iPJD79rsoee59HhEy18rtRdrhEUJ").unwrap();
        
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get blockhash");

        // Get balance of dev wallet
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        // Create a test transaction message to calculate fees
        let message = Message::new_with_blockhash(
            &[system_instruction::transfer(
                &keypair.pubkey(),
                &to_pubkey,
                balance,
            )],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        // Calculate exact fee for the message
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        // Deduct fee from lamports amount and create a transaction with corrected balance
        let transaction = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &keypair.pubkey(),
                &to_pubkey,
                balance - fee,
            )],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
