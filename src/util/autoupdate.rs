use std::io::Write;
use std::os::unix::fs::PermissionsExt;

pub async fn check_update(
) -> Result<bool, reqwest::Error> {

    let client = reqwest::Client::new();

    let response = client.get("https://api.github.com/repos/muffonlabs/mufbot-dc/releases/latest")
        .header("User-Agent", "mufbot-dc") // Required by GitHub API, else it will reject the request
        .send().await?;

    let json = response
        .json::<serde_json::Value>()
        .await?;

    let latest_version = json
        ["tag_name"]
        .as_str()
        .unwrap();

    let current_version =
        env!("CARGO_PKG_VERSION");

    if latest_version != current_version
    {

        println!("Update available! Current version: {}, Latest version: {}", current_version, latest_version);
    }

    Ok(latest_version
        != current_version)
}

pub async fn update(
) -> Result<bool, reqwest::Error> {

    let client = reqwest::Client::new();

    let response = client.get("https://api.github.com/repos/muffonlabs/mufbot-dc/releases/latest")
        .header("User-Agent", "mufbot-dc") // Required by GitHub API, else it will reject the request
        .send().await?;

    let json = response
        .json::<serde_json::Value>()
        .await?;

    let download_url = json["assets"]
        [0]["browser_download_url"]
        .as_str();

    if download_url.is_none() {

        println!("Failed to get download URL");

        return Ok(false);
    }

    let download_url =
        download_url.unwrap();

    let response = client
        .get(download_url)
        .header(
            "User-Agent",
            "mufbot-dc",
        ) // Required by GitHub API, else it will reject the request
        .send()
        .await?;

    let bytes =
        response.bytes().await?;

    let path = std::path::Path::new(
        "mufbot-dc",
    );

    let rm_res =
        std::fs::remove_file(path);

    if rm_res.is_err() {

        println!("Failed to remove old binary: {:?}", rm_res.err().unwrap());
    }

    let file =
        std::fs::File::create(path);

    if file.is_err() {

        println!("Failed to create new binary: {:?}", file.err().unwrap());

        Ok(false)
    } else {

        let mut file = file.unwrap();

        let write_res =
            file.write_all(&bytes);

        if write_res.is_err() {

            println!("Failed to write to new binary: {:?}", write_res.err().unwrap());

            return Ok(false);
        }

        // set permissions
        let mut perms = file
            .metadata()
            .unwrap()
            .permissions();

        perms.set_mode(0o755);

        let perm_res =
            file.set_permissions(perms);

        if perm_res.is_err() {

            println!("Failed to set permissions: {:?}", perm_res.err().unwrap());

            return Ok(false);
        }

        let cmd_res = std::process::Command::new("systemctl")
            .arg("restart")
            .arg(crate::env::SERVICE_NAME.as_str())
            .output();

        if cmd_res.is_err() {

            println!("Failed to restart service: {:?}", cmd_res.err().unwrap());

            return Ok(false);
        }

        Ok(true)
    }
}
