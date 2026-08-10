#![allow(unused, clippy::all)]
use std::{fs::read_to_string, process::Command, time::Duration};

use colored::Colorize;

struct Info {
    os: String,
    ip_addr: String,
    cpu_model: String,
    kernel: String,
    uptime: i32,
    energy_rate: String,
    time_left: Option<i32>,
    disks: Option<Vec<Disk>>,
}
impl Info {
    fn new() -> Self {
        Self {
            os: get_os(),
            ip_addr: get_ip_addr(),
            cpu_model: get_cpu_model(),
            kernel: get_kernel(),
            uptime: get_uptime(),
            energy_rate: get_rate(),
            time_left: get_remaining_time(),
            disks: get_disks(),
        }
    }
}

fn get_ip_addr() -> String {
    let output_raw = Command::new("ip").arg("a").output();
    let output_str = match output_raw {
        Ok(output) => String::from_utf8_lossy(&output.stdout).into_owned(),
        Err(_) => return String::from("failed"),
    };

    match output_str.lines().map(str::trim).find(|line| {
        line.starts_with("inet") && !line.contains("127.0.0.1") && !line.starts_with("inet6")
    }) {
        Some(output_line) => {
            return output_line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .split('/')
                .next()
                .unwrap()
                .to_string();
        }
        None => return "None".to_string(),
    }
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
fn get_kernel() -> String {
    match Command::new("uname").arg("-r").output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(e) => format!("Failed: {}", e.to_string()),
    }
}
fn get_uptime() -> i32 {
    let raw_uptime = match read_to_string("/proc/uptime") {
        Ok(uptime) => uptime
            .split_whitespace()
            .nth(0)
            .and_then(|secs| secs.split('.').nth(0))
            .unwrap()
            .to_owned(),
        Err(e) => return 0,
    };
    let total_seconds: i32 = raw_uptime.parse().unwrap();
    total_seconds
}
fn format_secs(secs: i32) -> (i32, i32) {
    ((secs / 3600), ((secs % 3600) / 60))
}
fn get_rate() -> String {
    match Command::new("upower").arg("-d").output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout)
            .lines()
            .find(|line| line.trim().starts_with("energy-rate"))
            .and_then(|line| line.split(':').nth(1).map(str::trim))
            .unwrap()
            .to_string(),
        Err(e) => format!("Failed: {}", e.to_string()),
    }
}
struct Disk {
    partition: String,
    size: String,
    used: String,
    avail: String,
    used_percent: String,
    mount: String,
}
fn get_disks() -> Option<Vec<Disk>> {
    let output_str = match Command::new("df").arg("-h").output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(_) => return None,
    };
    let mut lines = Vec::new();
    for line in output_str.lines() {
        if line.starts_with("/dev") {
            lines.push(line);
        }
    }
    let mut disks: Vec<Disk> = lines
        .iter()
        .map(|line| {
            let props: Vec<&str> = line.split_whitespace().collect();
            Disk {
                partition: props[0].to_string(),
                size: props[1].to_string(),
                used: props[2].to_string(),
                avail: props[3].to_string(),
                used_percent: props[4].to_string(),
                mount: props[5].to_string(),
            }
        })
        .collect();
    Some(disks)
}

fn get_remaining_time() -> Option<i32> {
    let usage: i32 = match read_to_string("/sys/class/power_supply/BAT1/power_now") {
        Ok(watts) => watts.trim().parse().unwrap(),
        Err(_) => return None,
    };
    let remaining_bat: i32 = match read_to_string("/sys/class/power_supply/BAT1/energy_now") {
        Ok(whs) => whs.trim().parse().unwrap(),
        Err(_) => return None,
    };
    let hours: f64 = remaining_bat as f64 / usage as f64;
    Some((hours * 3600.0) as i32)
}
fn main() {
    let info = Info::new();
    let (uptime_hrs, uptime_mins) = format_secs(info.uptime);
    let (remaining_hrs, remaining_mins) = format_secs(info.time_left.unwrap_or(0));
    let prints = vec![
        "OS: ".blue().to_string(),
        info.os,
        "\n".to_string(),
        "Kernel: ".blue().to_string(),
        info.kernel,
        "\n".to_string(),
        "Ip: ".blue().to_string(),
        info.ip_addr,
        "\n".to_string(),
        "Cpu: ".blue().to_string(),
        info.cpu_model,
        "\n".to_string(),
        "Uptime: ".blue().to_string(),
        (uptime_hrs.to_string()),
        " hours ".to_string(),
        (uptime_mins.to_string()),
        " minutes".to_string(),
        "\n".to_string(),
        "Energy rate: ".blue().to_string(),
        info.energy_rate,
        "\n".to_string(),
        "Remaining battery time: ".blue().to_string(),
        (remaining_hrs.to_string()),
        " hours ".to_string(),
        (remaining_mins.to_string()),
        " minutes".to_string(),
        "\n\n".to_string(),
    ];
    for thing in prints {
        print!("{}", thing);
    }

    match info.disks {
        Some(disklist) => {
            for (index, disk) in disklist.iter().enumerate() {
                let diskprints = vec![
                    "Disk ".blue().to_string(),
                    (index + 1).to_string().blue().to_string(),
                    ": ".blue().to_string(),
                    disk.partition.bright_cyan().to_string(),
                    "\nUsed: ".red().to_string(),
                    disk.used.to_owned(),
                    "/".to_string(),
                    disk.size.to_owned(),
                    " (".to_string(),
                    disk.used_percent.yellow().to_string(),
                    ")".to_string(),
                    " | ".to_string(),
                    "Free: ".green().to_string(),
                    disk.avail.to_owned(),
                    " | ".to_string(),
                    "Mountpoint: ".purple().to_string(),
                    disk.mount.to_owned(),
                    "\n\n".to_string(),
                ];
                for thing in diskprints {
                    print!("{thing}");
                }
            }
        }
        None => {}
    }
}
