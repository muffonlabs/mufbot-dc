use std::io::Write;

pub async fn check_update() -> Result<bool, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client.get("https://api.github.com/repos/muffonlabs/mufbot-dc/releases/latest")
        .header("User-Agent", "mufbot-dc") // Required by GitHub API, else it will reject the request
        .send().await?;

    let json = response.json::<serde_json::Value>().await?;

    let latest_version = json["tag_name"].as_str().unwrap();

    let current_version = env!("CARGO_PKG_VERSION");

    if latest_version != current_version {
        println!("Update available! Current version: {}, Latest version: {}", current_version, latest_version);
    }

    Ok(latest_version != current_version)
}

pub async fn update() -> Result<bool, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client.get("https://api.github.com/repos/muffonlabs/mufbot-dc/releases/latest")
        .header("User-Agent", "mufbot-dc") // Required by GitHub API, else it will reject the request
        .send().await?;

    let json = response.json::<serde_json::Value>().await?;

    let download_url = json["assets"][0]["browser_download_url"].as_str().unwrap();

    let response = client.get(download_url)
        .header("User-Agent", "mufbot-dc") // Required by GitHub API, else it will reject the request
        .send().await?;

    let bytes = response.bytes().await?;

    let path = std::path::Path::new("mufbot-dc");

    std::fs::remove_file(path).unwrap();

    let mut file = std::fs::File::create(path).unwrap();

    file.write_all(&bytes).unwrap();

    std::process::Command::new("systemctl")
        .arg("restart")
        .arg(crate::env::SERVICE_NAME.as_str())
        .output()
        .unwrap();

    Ok(true)
}