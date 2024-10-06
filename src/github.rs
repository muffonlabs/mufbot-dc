use serde_json::Value;

pub async fn start_rollout(
    version: &str
) -> Result<(), reqwest::Error> {
    let params_str = format!(
        r#"{{
            "ref": "main",
            "inputs": {{
                "version": "{}"
            }}
        }}"#,
        version
    );

    let params: Value =
        serde_json::from_str(
            params_str.as_str()
        )
        .unwrap();

    let client = reqwest::Client::new();

    let response = client.post(crate::env::GITHUB_WORKFLOW_URL.as_str())
        .json(&params)
        .header("User-Agent", "mufbot-dc") // Required by GitHub API, else it will reject the request
        .header("Authorization", format!("Bearer {}", crate::env::GITHUB_TOKEN.as_str()))
        .header("Accept", "application/vnd.github+json")
        .send().await?;

    if response.status().is_success() {
        return Ok(());
    }

    println!(
        "Failed to start rollout: {:?}",
        response
    );

    Err(response
        .error_for_status()
        .unwrap_err())
}
