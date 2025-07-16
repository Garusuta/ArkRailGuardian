use crate::{
    config::parser::parser_config,
    utils::killer::{get_process_by_name, kill_process_by_pid},
};
use std::{error::Error, fs, path::PathBuf, process::Command};
use sysinfo::System;

pub struct MuMu {
    version: String,
    path: PathBuf,
}

impl MuMu {
    pub fn new() -> Self {
        let app_config = parser_config().unwrap();
        MuMu {
            version: app_config.android_emulator.emulator_type,
            path: PathBuf::from(app_config.android_emulator.path),
        }
    }

    pub fn get_instance(&self, port: &str) -> Result<Vec<String>, Box<dyn Error>> {
        match self.version.as_str() {
            "MuMu5" => {
                let vms_dir = self.path.join("vms");
                for entry in fs::read_dir(vms_dir)? {
                    let entry = entry?;
                    let vm_config_path = entry.path().join("configs").join("vm_config.json");
                    let vm_config_content = fs::read_to_string(vm_config_path)?;
                    let vm_config_json: serde_json::Value =
                        serde_json::from_str(&vm_config_content)?;
                    let host_port =
                        vm_config_json["vm"]["nat"]["port_forward"]["adb"]["host_port"].as_str();
                    if let Some(p) = host_port {
                        if p == port {
                            let instance_name = entry.file_name().to_string_lossy().to_string();
                            let instance_id =
                                instance_name.split('-').last().unwrap_or("").to_string();
                            return Ok(vec![instance_name, instance_id]);
                        }
                    }
                }
                Err("No matching instance found".into())
            }
            _ => Err(format!("Unsupported MuMu version: {}", self.version).into()),
        }
    }

    pub fn kill_instance(&self, port: &str) -> Result<(), Box<dyn Error>> {
        match self.version.as_str() {
            "MuMu5" => {
                let instance = self.get_instance(port)?;
                let sys = System::new_all();

                // 尝试结束 MuMuNxDevice.exe 进程
                let device_processes = get_process_by_name(&sys, "MuMuNxDevice.exe")
                    .ok_or_else(|| "Failed to get MuMuNxDevice.exe processes")?;
                if !device_processes.is_empty() {
                    for process in &device_processes {
                        let cmdline = process.cmd();
                        // 初始实例不带 -v 参数
                        if &instance[1] == "0" {
                            if cmdline.len() < 2 {
                                kill_process_by_pid(process.pid())?;
                            }
                        } else {
                            if cmdline[2].eq_ignore_ascii_case(&instance[1]) {
                                kill_process_by_pid(process.pid())?;
                            }
                        }
                    }
                } else {
                    return Err(format!(
                        "No MuMuNxDevice.exe processes found for instance: {}",
                        instance[0]
                    )
                    .into());
                }

                // 尝试结束 MuMuVMMHeadless.exe 进程
                let headless_processes = get_process_by_name(&sys, "MuMuVMMHeadless.exe")
                    .ok_or_else(|| "Failed to get MuMuVMMHeadless.exe processes")?;
                if !headless_processes.is_empty() {
                    for process in &headless_processes {
                        let cmdline = process.cmd();
                        if cmdline[2].eq_ignore_ascii_case(&instance[0]) {
                            kill_process_by_pid(process.pid())?;
                        }
                    }
                } else {
                    return Err(format!(
                        "No MuMuVMMHeadless.exe processes found for instance: {}",
                        instance[0]
                    )
                    .into());
                }
                Ok(())
            }
            _ => Err(format!("Unsupported MuMu version: {}", self.version).into()),
        }
    }

    pub fn open_instance(&self, id: &str) -> Result<(), Box<dyn Error>> {
        match self.version.as_str() {
            "MuMu5" => {
                let status = Command::new("cmd")
                    .current_dir(self.path.join("nx_device").join("12.0").join("shell"))
                    .arg("/c")
                    .arg("MuMuNxDevice.exe")
                    .arg("-v")
                    .arg(id)
                    .spawn()?;
                println!("Launched MuMu instance {} (PID: {:?})", id, status.id());
                Ok(())
            }
            _ => Err(format!("Unsupported MuMu version: {}", self.version).into()),
        }
    }
}
