use super::error;
use std::str::FromStr;
use std::string::ToString;

const PARITY_CLIENT_CMD: &str = "parity";
const GETH_CLIENT_CMD: &str = "geth";

pub enum SupportedBlockchainClients {
  Parity,
  Geth,
}

impl FromStr for SupportedBlockchainClients {
  type Err = error::NodeError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      PARITY_CLIENT_CMD => Ok(SupportedBlockchainClients::Parity),
      GETH_CLIENT_CMD => Ok(SupportedBlockchainClients::Geth),
      _ => Err(error::NodeError::UnsupportedClient),
    }
  }
}

impl ToString for SupportedBlockchainClients {
  fn to_string(&self) -> String {
    match self {
      SupportedBlockchainClients::Parity => PARITY_CLIENT_CMD.to_string(),
      SupportedBlockchainClients::Geth => GETH_CLIENT_CMD.to_string(),
    }
  }
}

pub fn default_options_from(client: SupportedBlockchainClients) -> Vec<String> {
  match client {
    SupportedBlockchainClients::Parity => {
      vec![
        "--config".to_string(),
        "dev".to_string(),
        "--ws-origins".to_string(),
        "all".to_string()
      ]
    },
    SupportedBlockchainClients::Geth => {
      vec![
        "--dev".to_string(),
        "--rpc".to_string(),
        "--ws".to_string(),
        "--wsorigins".to_string(),
        "*".to_string()
      ]
    },
  }
}
