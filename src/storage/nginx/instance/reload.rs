use tokio::process::Command;

pub async fn reload() -> Result<String, String> {
    let output = Command::new("nginx")
        .arg("-s")
        .arg("reload")
        .output()
        .await
        .unwrap();

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    return Err(String::from_utf8_lossy(&output.stdout).to_string());
}