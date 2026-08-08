#![allow(unused, clippy::all)]
use std::{fs::read_to_string, process::Command};

struct Info {
    os: String,
    ip_addr: String,
    cpu_model: String,
}
impl Info {
    fn new() -> Self {
        Self {
            os: get_os(),
            ip_addr: get_ip_addr(),
            cpu_model: get_cpu_model(),
        }
    }
}

fn get_ip_addr() -> String {
    let output_raw = Command::new("ip").arg("a").output();
    let output_str = match output_raw {
        Ok(output) => String::from_utf8_lossy(&output.stdout).into_owned(),
        Err(_) => return String::from("failed"),
    };

    output_str
        .lines()
        .map(str::trim)
        .find(|line| {
            line.starts_with("inet") && !line.contains("127.0.0.1") && !line.starts_with("inet6")
        })
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split('/')
        .next()
        .unwrap()
        .to_string()
}
fn get_cpu_model() -> String {
    let output_raw = Command::new("lscpu").output();
    let output_str = match output_raw {
        Ok(output) => String::from_utf8_lossy(&output.stdout).into_owned(),
        Err(e) => return format!("Failed: {}", e.to_string()),
    };
    output_str
        .lines()
        .find(|line| line.starts_with("Model name"))
        .and_then(|line| line.split(":").nth(1))
        .map(str::trim)
        .unwrap()
        .to_string()
}
fn get_os() -> String {
    let Ok(os_release) = read_to_string("/etc/os-release") else {
        return String::from("Failed");
    };

    os_release
        .lines()
        .find(|line| line.starts_with("PRETTY_NAME"))
        .and_then(|line| line.split("=").nth(1))
        .map(|line| line.trim_matches('"'))
        .unwrap_or("Linux")
        .to_string()
        + " "
        + std::env::consts::ARCH
}

fn main() {
    let info = Info::new();
    println!(
        "OS: {}\nIp: {}\nCpu: {}",
        info.os, info.ip_addr, info.cpu_model
    );
}
