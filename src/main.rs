mod config;

use base58::FromBase58;
use clap::Parser;
use near_seed_phrase::{derive_key, NearSecretKey};

fn main() -> anyhow::Result<()>{
    let config = config::Config::parse();
    match (config.input.seed, config.input.key) {
        (Some(seed), None) => {
            let secret_key = derive_key!(seed);
            println!("Signing/secret/private key: {}", secret_key);
            println!("Verifying/public key: {}", secret_key.to_public_key());        
            Ok(())
        }
        (None, Some(key)) => {
            let key_bytes = key.replace("ed25519:", "").from_base58().map_err(|_|anyhow::anyhow!("Base58 decoding error"))?;
            let secret_key = NearSecretKey::from_bytes(key_bytes.as_slice())?;
            println!("Verifying/public key: {}", secret_key.to_public_key());        
            Ok(())
        }
        _ => unreachable!(),
    }
}
