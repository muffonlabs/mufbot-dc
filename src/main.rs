mod db;
mod discord;
mod env;
mod github;
mod util;

use dotenv::dotenv;

#[tokio::main]

async fn main() {
    dotenv().ok();

    // conditional compilation for only unix because auto
    // update is not supported on non unix. for eg: windows
    #[cfg(target_family = "unix")]
    util::init_autoupdate().await;

    discord::initiate_bot().await;
}
