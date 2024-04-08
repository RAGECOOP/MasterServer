use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
  pub server: Server,
}

#[derive(Deserialize)]
pub struct Server {
  pub port: u16,
  pub workers: u16,
  pub address: String,
}
