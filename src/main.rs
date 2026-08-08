#![allow(unused, clippy::all)]
use std::{fs::read_to_string, process::Command};

struct Info {
    ip_addr: String,
    cpu_model: String,
}
impl Info {
    fn new() -> Self {
        Self {
            ip_addr: get_ip_addr(),
            cpu_model: get_cpu_model(),
        }
    }
}

fn get_ip_addr() -> String {
    let output_raw = Command::new("ip").arg("a").output();
    let mut output_str = match output_raw {
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
        .unwrap() // no need to clear this up either
        .split('/')
        .next()
        .unwrap()
        .to_string()
}
fn get_cpu_model() -> String {
    let Ok(output_str) = read_to_string("/proc/cpuinfo") else {
        return String::from("failed");
    };
    output_str
        .lines()
        .find(|line| line.starts_with("model name"))
        .unwrap() // guaranteed to be there so no problem using unwrap
        .split(": ")
        .nth(1)
        .unwrap()
        .to_string()
}

fn main() {
    let info = Info::new();
    println!("Ip: {} Cpu: {}", info.ip_addr, info.cpu_model);
}
