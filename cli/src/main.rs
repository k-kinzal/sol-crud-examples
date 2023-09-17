use clap::{Parser, Subcommand};
use client::Client;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        #[arg(short, long)]
        data: String,
    },
    Update {
        #[arg(short, long)]
        key: Pubkey,
        #[arg(short, long)]
        data: String,
    },
    Delete {
        #[arg(short, long)]
        key: Pubkey,
    },
    Get {
        #[arg(short, long)]
        key: Pubkey,

        #[arg(long, default_value = "false")]
        json: bool,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    json_rpc_url: String,
    websocket_url: String,
    keypair_path: PathBuf,
    address_labels: HashMap<String, String>,
    commitment: String,
}

fn read_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let yaml = fs::read_to_string(path)?;
    let config = serde_yaml::from_str::<Config>(&yaml)?;

    Ok(config)
}

fn read_key_pair<P: AsRef<Path>>(path: P) -> anyhow::Result<Keypair> {
    let json = fs::read_to_string(path)?;
    let bytes = serde_json::from_str::<Vec<u8>>(&json)?;
    let key_pair = Keypair::from_bytes(&bytes)?;

    Ok(key_pair)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config_path = cli
        .config
        .or_else(|| {
            env::var("HOME").ok().map(|v| {
                Path::new(&v)
                    .join(".config")
                    .join("solana")
                    .join("cli")
                    .join("config.yml")
            })
        })
        .unwrap();
    let config = read_config(config_path)?;
    let key_pair = read_key_pair(config.keypair_path)?;
    let client = Client::new(config.json_rpc_url, key_pair);

    match cli.command {
        Some(Commands::Create { data }) => {
            let value = serde_json::from_str::<serde_json::Value>(&data)?;
            let data = serde_json::to_vec(&value)?;
            let res = client.create(&data).await?;
            println!("{}", serde_json::to_string(&res)?);
        }
        Some(Commands::Update { key, data }) => {
            let value = serde_json::from_str::<serde_json::Value>(&data)?;
            let data = serde_json::to_vec(&value)?;
            let res = client.update(key, &data).await?;
            println!("{}", serde_json::to_string(&res)?);
        }
        Some(Commands::Delete { key }) => {
            let res = client.delete(key).await?;
            println!("{}", serde_json::to_string(&res)?);
        }
        Some(Commands::Get { key, json }) => {
            let res = client.get(key).await?;
            if json {
                let data = serde_json::from_slice::<serde_json::Value>(&res)?;
                println!("{}", serde_json::to_string(&data)?);
            } else {
                println!("{:?}", res);
            }
        }
        None => {}
    };

    Ok(())
}
