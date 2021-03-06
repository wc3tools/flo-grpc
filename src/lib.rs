pub use anyhow::{Error, Result};
pub use tonic::transport::{Channel, Endpoint};

pub mod player {
  tonic::include_proto!("player");
}

pub mod game {
  tonic::include_proto!("game");
}

pub mod node {
  tonic::include_proto!("node");
}

pub mod controller {
  tonic::include_proto!("controller");
}
