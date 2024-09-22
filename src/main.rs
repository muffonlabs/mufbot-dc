mod db;
mod discord;
mod env;
mod util;

use dotenv::dotenv;

#[tokio::main]

async fn main() {

    dotenv().ok();

    let update =
        util::autoupdate::check_update(
        )
        .await;

    if update.is_err() {

        println!("Failed to check for updates: {:?}", update.err().unwrap());
    } else if update.unwrap() {

        println!(
            "Attempting to update..."
        );

        let update =
            util::autoupdate::update()
                .await;

        if update.is_err() {

            println!("Failed to update: {:?}", update.err().unwrap());
        } else {

            println!(
                "Update successful"
            );
        }

        return;
    }

    discord::initiate_bot().await;
}
