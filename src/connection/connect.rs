use crate::constant::{connection::{GET_FOREGROUND_APP, LOCAL_HOST}, APP_CONFIG};
use std::{error::Error, process::Command};

pub fn connect_device(port: &str) -> Result<(), Box<dyn Error>> {
    let address = format!("{}:{}", LOCAL_HOST, port);
    let output = Command::new("cmd")
        .current_dir(APP_CONFIG.platform_tools.clone())
        .arg("/c")
        .args(["adb", "connect", &address])
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to connect to device at {}: {}",
            address,
            String::from_utf8_lossy(&output.stderr)
        ).into())
    }
}

pub fn get_foreground_app(address: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("cmd")
    .arg("/c")
    .arg("adb")
    .arg("-s")
    .arg(address)
    .arg("shell")
    .arg(GET_FOREGROUND_APP)
    .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(format!(
            "Failed to get foreground app from device at {}: {}",
            address,
            String::from_utf8_lossy(&output.stderr)
        ).into())
    }
}