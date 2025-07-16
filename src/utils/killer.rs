use std::{error::Error, process::Command};

use sysinfo::{Pid, Process, System};

pub fn find_process_by_port(port: &str) -> Option<String> {
    let port = format!(":{}", port);
    let output = Command::new("cmd")
        .arg("/c")
        .args(["netstat", "-ano", "|", "findStr", &port])
        .output()
        .ok()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            if let Some(pid) = stdout.split_whitespace().last() {
                return Some(pid.to_string());
            }
        }
    }
    None
}

pub fn get_process_by_name<'a>(sys: &'a System, name: &str) -> Option<Vec<&'a Process>> {
    let mut processes: Vec<&'a Process> = Vec::new();
    sys.processes()
        .iter()
        .filter(|(_, process)| process.name().eq_ignore_ascii_case(name))
        .for_each(|(_, process)| {
            processes.push(process);
        });
    if !processes.is_empty() {
        Some(processes)
    } else {
        None
    }
}

pub fn kill_process_by_name(names: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let sys = System::new_all();
    let mut failed_kills: Vec<String> = Vec::new();
    sys.processes()
        .iter()
        .filter(|(_, process)| names.iter().any(|n| process.name().eq_ignore_ascii_case(n)))
        .for_each(|(_, process)| {
            if process.kill() {
                println!("Killed process: {}", process.name().to_string_lossy());
                println!("Process cmdline: {:?}", process.cmd());
            } else {
                failed_kills.push(process.name().to_string_lossy().into_owned());
            }
        });

    if !failed_kills.is_empty() {
        Err(format!("Failed to kill the following processes: {:?}", failed_kills).into())
    } else {
        Ok(())
    }
}

pub fn kill_process_by_pid(pid: Pid) -> Result<(), Box<dyn Error>> {
    let sys = System::new_all();
    for (p, process) in sys.processes() {
        if *p == pid {
            if process.kill() {
                println!("Killed process with PID: {}", pid);
                return Ok(());
            } else {
                return Err(format!("Failed to kill process with PID: {}", pid).into());
            }
        }
    }
    Err(format!("No process found with PID: {}", pid).into())
}
