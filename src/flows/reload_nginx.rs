use tokio::process::Command;

use crate::settings::SettingsReader;

pub async fn reload_nginx(settings_reader: &SettingsReader) -> Result<String, String> {
    if !settings_reader.get_start_nginx().await {
        return Err("Nginx is not started".to_string());
    }

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
