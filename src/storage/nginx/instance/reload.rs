use tokio::process::Command;

pub async fn reload() -> Result<String, String> {
    let test_output = Command::new("nginx").arg("-t").output().await.unwrap();

    println!("Nginx Test result: {:?}", test_output);
    if !test_output.status.success() {
        return Err(String::from_utf8_lossy(&test_output.stderr).to_string());
    }

    let output = Command::new("nginx")
        .arg("-s")
        .arg("reload")
        .output()
        .await
        .unwrap();

    println!("Nginx Reload result: {:?}", output);
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    return Ok(String::from_utf8_lossy(&test_output.stdout).to_string());
}
