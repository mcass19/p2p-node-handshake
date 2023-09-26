use std::{
    fmt::{self, Display},
    time::Duration,
};
use tokio::{
    task::{JoinError, JoinHandle},
    time::error::Elapsed,
};

pub mod btc;

pub async fn perform_handshake(
    node_addresses: Vec<String>,
    user_agent: String,
) -> Result<Vec<P2PResult>, P2PError> {
    let mut results = Vec::new();
    let joins: Vec<(String, JoinHandle<Result<String, P2PError>>)> = node_addresses
        .iter()
        .map(|node_addr| {
            let owned_node_addr = node_addr.to_owned();
            let owned_user_agent = user_agent.to_owned();

            let join = tokio::spawn(async {
                tokio::time::timeout(
                    Duration::from_secs(10), // arbitrary number, could be configurable
                    btc::perform_bitcoin_handshake(owned_node_addr, owned_user_agent),
                )
                .await?
            });

            (node_addr.to_owned(), join)
        })
        .collect();

    for (address, join) in joins {
        let res = join.await?;
        results.push(P2PResult::new(address, res));
    }

    Ok(results)
}

// ---
// P2P Error
#[derive(Debug)]
pub struct P2PError {
    message: String,
}

impl fmt::Display for P2PError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "P2P Error: {}", self.message)
    }
}

impl From<std::io::Error> for P2PError {
    fn from(err: std::io::Error) -> Self {
        P2PError {
            message: err.to_string(),
        }
    }
}

impl From<bitcoin::consensus::encode::Error> for P2PError {
    fn from(err: bitcoin::consensus::encode::Error) -> Self {
        P2PError {
            message: err.to_string(),
        }
    }
}

impl From<JoinError> for P2PError {
    fn from(err: JoinError) -> Self {
        P2PError {
            message: err.to_string(),
        }
    }
}

impl From<Elapsed> for P2PError {
    fn from(err: Elapsed) -> Self {
        P2PError {
            message: err.to_string(),
        }
    }
}

// ---
// P2P Result
pub struct P2PResult {
    address: String,
    result: Result<String, P2PError>,
}

impl P2PResult {
    pub fn new(address: String, result: Result<String, P2PError>) -> P2PResult {
        P2PResult { address, result }
    }

    pub fn address(&self) -> &str {
        self.address.as_ref()
    }

    pub fn result(&self) -> Result<&String, &P2PError> {
        self.result.as_ref()
    }
}

impl Display for P2PResult {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.result.is_ok() {
            true => {
                write!(
                    fmt,
                    "{} {}: {}",
                    "\u{2705}",
                    self.address,
                    self.result().unwrap()
                )
            }
            false => {
                write!(
                    fmt,
                    "{} {}: {}",
                    "\u{274C}",
                    self.address,
                    self.result().err().unwrap()
                )
            }
        }
    }
}
