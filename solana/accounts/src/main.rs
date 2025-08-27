use solana_sdk::signer::{keypair::{self, Keypair}, Signer};

#[tokio::main]
async fn main() {
    let keypair = Keypair::new();
    println!("public key: {}", keypair.pubkey());
    println!("Secret key: {:?}", keypair.to_bytes());
}