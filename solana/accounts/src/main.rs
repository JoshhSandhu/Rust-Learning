use solana_sdk::signer::{keypair::{self, Keypair}, Signer};
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

#[tokio::main]
async fn main() {
    let program_address = pubkey!("11111111111111111111111111111111");
    let seeds = [b"helloworld".as_ref()];
    let (pda, bump) = Pubkey::find_program_address(&seeds, &program_address);
    println!("PDA: {}",pda);
    println!("Seeds: {:#?}", seeds);
    let keypair = Keypair::new();
    println!("public key: {}", keypair.pubkey());
    println!("Secret key: {:?}", keypair.to_bytes());
}