mod config;

use base58::{FromBase58, ToBase58};
use clap::Parser;
use near_seed_phrase::{derive_key, NearSecretKey, NearSeedPhrase};
use serde_json::json;
use std::process::{Command, Stdio};

fn main() -> anyhow::Result<()> {
    let config = config::Config::parse();
    match (&config.input.seed, &config.input.key, &config.input.create) {
        (Some(seed), None, None) => {
            let secret_key = derive_key!(seed);
            println!("Secret key: ed25519:{}", secret_key.to_bytes().to_base58());
            println!("Key pair: {}", secret_key);
            println!("Public key: {}", secret_key.to_public_key());
            Ok(())
        }
        (None, Some(key), None) => {
            let key_bytes = key
                .replace("ed25519:", "")
                .from_base58()
                .map_err(|_| anyhow::anyhow!("Base58 decoding error"))?;
            println!("{:?}", key_bytes.len());
            let secret_key = NearSecretKey::from_keypair_bytes(key_bytes.as_slice())?;
            println!("Key pair: {}", secret_key);
            println!("Public key: {}", secret_key.to_public_key());
            Ok(())
        }
        (None, None, Some(name)) => {
            // Generate a new seed and derive the secret key
            let seed = NearSeedPhrase::generate(12)?;
            let secret_key = derive_key!(seed);
            // Generate JSON of the new 1Password item
            let mut item = json!({
                "title": name,
                "category": "CUSTOM",
                "category_id": "115",
                "fields": [{
                    "id": "publicKey",
                    "type": "TEXT",
                    "label": "Public Key",
                    "value": secret_key.to_public_key().to_string()
                },{
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
                    "id": "keyPair",
                    "type": "CONCEALED",
                    "label": "Key Pair",
                    "value": secret_key.to_string()
                }]
            });
            // if present, add account id and key pair JSON fields
            if let Some(account) = config.account {
                let key_pair_json = json!({
                    "id": "keyPairJson",
                    "type": "CONCEALED",
                    "label": "Key Pair JSON",
                    "value": json!({
                        "account_id": account,
                        "public_key": secret_key.to_public_key().to_string(),
                        "private_key": secret_key.to_string()
                    }).to_string()
                });
                let account = json!({
                    "id": "accountId",
                    "type": "TEXT",
                    "label": "Account ID",
                    "value": account
                });
                let fields = item
                    .as_object_mut()
                    .unwrap()
                    .get_mut("fields")
                    .unwrap()
                    .as_array_mut()
                    .unwrap();
                fields.insert(0, account);
                fields.push(key_pair_json);
            }
            let input = Command::new("echo")
                .arg(item.to_string())
                .stdout(Stdio::piped())
                .spawn()?;
            let command = Command::new("op")
                .arg("item")
                .arg("create")
                .arg("--vault")
                .arg(&config.vault.unwrap())
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
