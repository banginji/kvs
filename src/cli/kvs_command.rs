use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    name = "kvs-cli",
    version,
    author,
    about = "issue commands"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: KvsCliCommand
}

#[derive(Subcommand, Debug, Clone)]
pub enum KvsCliCommand {
    Get {
        key: String
    },
    Set {
        key: String,
        value: String
    },
    Delete {
        key: String
    },
    Unknown
}

impl KvsCliCommand {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        match bytes {
            _ if bytes.starts_with(b"get") => {
                let cmd_string = String::from_utf8_lossy(&bytes[..]);
                let split_parts: Vec<&str> = cmd_string.split_whitespace().collect();
                KvsCliCommand::Get { key: split_parts[1].to_string() }
            },
            _ if bytes.starts_with(b"set") => {
                let cmd_string = String::from_utf8_lossy(&bytes[..]);
                let split_parts: Vec<&str> = cmd_string.split_whitespace().collect();
                KvsCliCommand::Set { 
                    key: split_parts[1].to_string(),
                    value: split_parts[2].to_string() 
                }
            },
            _ if bytes.starts_with(b"delete") => {
                let cmd_string = String::from_utf8_lossy(&bytes[..]);
                let split_parts: Vec<&str> = cmd_string.split_whitespace().collect();
                KvsCliCommand::Delete { key: split_parts[1].to_string() }
            },
            _ => {
                println!("unknown {:?}", bytes);
                KvsCliCommand::Unknown
            }
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            KvsCliCommand::Get { key } => format!("get {}", key).into_bytes(),
            KvsCliCommand::Set { key, value } => format!("set {} {}", key, value).into_bytes(),
            KvsCliCommand::Delete { key } => format!("delete {}", key).into_bytes(),
            KvsCliCommand::Unknown => b"Unknown".to_vec()
        }
    }
}
