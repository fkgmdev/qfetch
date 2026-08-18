use std::{fs::read_to_string, process::Command};

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
    battery_status: BatteryStatus,
}
// enum InfoError {
//     CommandFail(String),
//     IoError(std::io::Error),
//     ParseError(String),
// }
impl Info {
    fn new() -> Self {
        Self {
            os: match get_os() {
                Ok(Some(os)) => os,
                Ok(None) => "Linux".to_string(),
                Err(e) => format!("Failed: {e}"),
            },
            ip_addr: match get_ip_addr() {
                Ok(Some(addr)) => addr,
                Ok(None) => "None".to_string(),
                Err(e) => format!("Failed: {e}"),
            },
            cpu_model: match get_cpu_model() {
                Ok(Some(model)) => model,
                Ok(None) => "None".to_string(),
                Err(e) => format!("Failed: {e}"),
            },
            kernel: get_kernel(),
            uptime: get_uptime(),
            energy_rate: get_rate(),
            time_left: get_remaining_time(),
            disks: get_disks(),
            battery_status: get_battery_status(),
        }
    }
}

fn get_ip_addr() -> Result<Option<String>, String> {
    let output_raw = Command::new("ip")
        .arg("a")
        .output()
        .map_err(|e| e.to_string())?;
    let output_str = String::from_utf8_lossy(&output_raw.stdout).to_string();
    Ok(output_str
        .lines()
        .map(str::trim)
        .find(|line| {
            line.starts_with("inet") && !line.contains("127.0.0.1") && !line.starts_with("inet6")
        })
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|ip| ip.split("/").next())
        .map(|ip| ip.to_string()))
}
fn get_cpu_model() -> Result<Option<String>, String> {
    let output_raw = Command::new("lscpu").output().map_err(|e| e.to_string())?;
    let output_str = String::from_utf8_lossy(&output_raw.stdout).to_string();
    Ok(output_str
        .lines()
        .find(|line| line.starts_with("Model name"))
        .and_then(|line| line.split(":").nth(1))
        .map(str::trim)
        .map(|s| s.to_string()))
}
fn get_os() -> Result<Option<String>, String> {
    let os_release = read_to_string("/etc/os-release").map_err(|e| e.to_string())?;

    Ok(os_release
        .lines()
        .find(|line| line.starts_with("PRETTY_NAME"))
        .and_then(|line| line.split("=").nth(1))
        .map(|line| line.trim_matches('"').to_string() + " " + std::env::consts::ARCH))
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
#[derive(PartialEq)]
enum BatteryStatus {
    Charging,
    Discharging,
    Limbo,
}
fn get_battery_status() -> BatteryStatus {
    match read_to_string("/sys/class/power_supply/BAT1/status") {
        Ok(status) => {
            if status.trim() == "Discharging" {
                BatteryStatus::Discharging
            } else if status.trim() == "Charging" {
                BatteryStatus::Charging
            } else {
                BatteryStatus::Limbo
            }
        }
        Err(_) => BatteryStatus::Limbo,
    }
}

pub fn fetch() -> Vec<(String, String)> {
    let info = Info::new();
    let (uptime_hrs, uptime_mins) = format_secs(info.uptime);
    let (remaining_hrs, remaining_mins) = format_secs(info.time_left.unwrap_or(0));
    let mut out = vec![
        ("OS: ".blue().to_string(), info.os),
        ("Kernel: ".blue().to_string(), info.kernel),
        ("Ip: ".blue().to_string(), info.ip_addr),
        ("Cpu: ".blue().to_string(), info.cpu_model),
        (
            "Uptime: ".blue().to_string(),
            format!("{} hours {} minutes", uptime_hrs, uptime_mins),
        ),
    ];
    if info.battery_status == BatteryStatus::Discharging {
        out.push(("Energy rate: ".blue().to_string(), info.energy_rate));
        out.push((
            "Time left: ".blue().to_string(),
            format!("{} hours {} minutes", remaining_hrs, remaining_mins),
        ));
    }
    if let Some(disks) = info.disks {
        for (index, disk) in disks.iter().enumerate() {
            let diskprints = (
                ("\nDisk ".blue().to_string()
                    + &(index + 1).to_string().blue().to_string()
                    + &": ".blue().to_string()
                    + &disk.partition.bright_cyan().to_string()),
                ("\nUsed: ".red().to_string()
                    + &disk.used
                    + "/"
                    + &disk.size
                    + " ("
                    + &disk.used_percent.yellow().to_string()
                    + ")"
                    + " | "
                    + &"Free: ".green().to_string()
                    + &disk.avail
                    + " | "
                    + &"Mountpoint: ".purple().to_string()
                    + &disk.mount),
            );
            out.push(diskprints);
        }
    }
    out
}
