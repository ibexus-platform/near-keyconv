mod config;

use std::{
    process::{Command, Stdio},
};
use base58::{FromBase58, ToBase58};
use clap::Parser;
use near_seed_phrase::{derive_key, NearSecretKey, NearSeedPhrase};

fn main() -> anyhow::Result<()> {
    let config = config::Config::parse();
    match (
        config.input.seed,
        config.input.key,
        config.input.create,
        config.vault,
    ) {
        (Some(seed), None, None, None) => {
            let secret_key = derive_key!(seed);
            println!("Secret key: ed25519:{}", secret_key.to_bytes().to_base58());
            println!("Public key: {}", secret_key.to_public_key());
            Ok(())
        }
        (None, Some(key), None, None) => {
            let key_bytes = key
                .replace("ed25519:", "")
                .from_base58()
                .map_err(|_| anyhow::anyhow!("Base58 decoding error"))?;
            println!("{:?}", key_bytes.len());
            let secret_key = NearSecretKey::from_keypair_bytes(key_bytes.as_slice())?;
            println!("Public key: {}", secret_key.to_public_key());
            Ok(())
        }
        (None, None, Some(name), Some(vault)) => {
            let seed = NearSeedPhrase::generate(12)?;
            let secret_key = derive_key!(seed);
            let item = serde_json::json!({
            "title": name,
            "category": "CUSTOM",
            "category_id": "115",
            "fields": [{
                "id": "seed",
                "type": "CONCEALED",
                "label": "Seed",
                "value": seed.to_string()
            },{
                "id": "secretKey",
                "type": "CONCEALED",
                "label": "Secret Key",
                "value": "ed25519:".to_string() + secret_key.to_bytes().to_base58().as_str()
            },{
                "id": "publicKey",
                "type": "TEXT",
                "label": "Public Key",
                "value": secret_key.to_public_key().to_string()
            }]})
            .to_string();
            let input = Command::new("echo")
                .arg(item)
                .stdout(Stdio::piped())
                .spawn()?;
            let command = Command::new("op")
                .arg("item")
                .arg("create")
                .arg("--vault")
                .arg(vault)
                .stdin(Stdio::from(input.stdout.unwrap()))
                .stdout(Stdio::piped())
                .spawn()?;
            let _output = command.wait_with_output()?;
            // std::io::stdout().write_all(&output.stdout).unwrap();
            // std::io::stderr().write_all(&output.stderr).unwrap();
            Ok(())
        }
        _ => unreachable!(),
    }
}
