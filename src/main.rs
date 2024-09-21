mod db;
mod discord;
mod env;

use dotenv::dotenv;

#[tokio::main]

async fn main() {

    dotenv().ok();

    discord::initiate_bot().await;
}
