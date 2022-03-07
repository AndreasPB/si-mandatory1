use mongodb::{options::ClientOptions, Client};

#[derive(Debug, Clone)]
pub struct SharedState {
    pub db: mongodb::Client,
}

impl SharedState {
    pub async fn new() -> tide::Result<Self> {
        let client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await?;
        let client = Client::with_options(client_options)?;
        Ok(Self { db: client })
    }

    pub fn get_db(&self) -> &mongodb::Client {
        &self.db
    }
}
