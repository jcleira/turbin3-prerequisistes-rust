// Import the programs module.
mod programs;

// Define a test module which is only included when testing.
#[cfg(test)]
mod tests {
    // Import necessary dependencies.
    use bs58;
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer, system_program};
    use solana_sdk::{
        message::Message,
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
    };
    use std::io::{self, BufRead};
    use std::str::FromStr;

    // Import specific items from the programs module.
    use crate::programs::wba_prereq::{CompleteArgs, WbaPrereqProgram};

    // Define the RPC URL for Solana's devnet.
    const RPC_URL: &str = "https://api.devnet.solana.com";

    /// Test function to enroll in a program.
    /// Reads a keypair, derives the program address, creates the necessary transaction,
    /// sends it to the blockchain, and prints the transaction signature.
    #[test]
    fn enroll() {
        let signer = read_keypair_file("wba-wallet.json").expect("Couldn't find wallet file");
        let prereq = WbaPrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);
        let args = CompleteArgs {
            github: b"jcleira".to_vec(),
        };
        let rpc_client = RpcClient::new(RPC_URL);
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = WbaPrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    /// Test function to generate a new keypair.
    /// Creates a new keypair and prints the public key and the keypair in byte array form.
    #[test]
    fn keygen() {
        let kp = Keypair::new();

        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    /// Test function to request an airdrop.
    /// Reads a keypair, requests an airdrop of 2 SOL to the public key, and prints the transaction signature.
    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

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
        };
    }

    /// Test function to transfer SOL.
    /// Reads a keypair, creates a transfer transaction to a specified public key, sends it, and prints the transaction signature.
    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("9PgAZaZjFbd74x75hyXJ6Xz6YTcWU1GU4vWWeU1Xmcfc").unwrap();

        let rpc_client = RpcClient::new(RPC_URL);

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    /// Test function to transfer all SOL.
    /// Reads a keypair, checks the balance, creates a transfer transaction for the entire balance minus fees,
    /// sends it, and prints the transaction signature.
    #[test]
    fn transfer_all_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("9PgAZaZjFbd74x75hyXJ6Xz6YTcWU1GU4vWWeU1Xmcfc").unwrap();

        let rpc_client = RpcClient::new(RPC_URL);

        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    /// Test function to convert a base58 encoded private key to a wallet file byte array.
    /// Reads a base58 string from standard input, decodes it, and prints the byte array.
    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();

        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");

        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    /// Test function to convert a wallet file byte array to a base58 encoded private key.
    /// Reads a byte array from standard input, encodes it as base58, and prints the string.
    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();

        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
}
