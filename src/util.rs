pub mod autoupdate;

#[cfg(target_family = "unix")]

pub async fn init_autoupdate() {

    let update =
        autoupdate::check_update()
            .await;

    if update.is_err() {

        println!("Failed to check for updates: {:?}", update.err().unwrap());
    } else if update.unwrap() {

        println!(
            "Attempting to update..."
        );

        let update =
            autoupdate::update().await;

        if update.is_err() {

            println!("Failed to update: {:?}", update.err().unwrap());
        } else {

            println!(
                "Update successful"
            );
        }

        return;
    }
}
