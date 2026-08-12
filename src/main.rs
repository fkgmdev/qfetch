#![allow(unused, clippy::all)]
// use colored::Colorize;
// use std::{fs::read_to_string, process::Command, time::Duration};
mod info;

fn main() {
    let sysinfo = info::fetch();
    for (t1, t2) in sysinfo {
        print!("{t1}{t2}\n");
    }
}

// fn main() {
//     let info = Info::new();
//     let (uptime_hrs, uptime_mins) = format_secs(info.uptime);
//     let (remaining_hrs, remaining_mins) = format_secs(info.time_left.unwrap_or(0));
//     let prints = vec![
//         "OS: ".blue().to_string(),
//         info.os,
//         "\n".to_string(),
//         "Kernel: ".blue().to_string(),
//         info.kernel,
//         "\n".to_string(),
//         "Ip: ".blue().to_string(),
//         info.ip_addr,
//         "\n".to_string(),
//         "Cpu: ".blue().to_string(),
//         info.cpu_model,
//         "\n".to_string(),
//         "Uptime: ".blue().to_string(),
//         (uptime_hrs.to_string()),
//         " hours ".to_string(),
//         (uptime_mins.to_string()),
//         " minutes".to_string(),
//         "\n".to_string(),
//         "Energy rate: ".blue().to_string(),
//         info.energy_rate,
//         "\n".to_string(),
//         "Remaining battery time: ".blue().to_string(),
//         (remaining_hrs.to_string()),
//         " hours ".to_string(),
//         (remaining_mins.to_string()),
//         " minutes".to_string(),
//         "\n\n".to_string(),
//     ];
//     for thing in prints {
//         print!("{}", thing);
//     }
//
//     match info.disks {
//         Some(disklist) => {
//             for (index, disk) in disklist.iter().enumerate() {
//                 let diskprints = vec![
//                     "Disk ".blue().to_string(),
//                     (index + 1).to_string().blue().to_string(),
//                     ": ".blue().to_string(),
//                     disk.partition.bright_cyan().to_string(),
//                     "\nUsed: ".red().to_string(),
//                     disk.used.to_owned(),
//                     "/".to_string(),
//                     disk.size.to_owned(),
//                     " (".to_string(),
//                     disk.used_percent.yellow().to_string(),
//                     ")".to_string(),
//                     " | ".to_string(),
//                     "Free: ".green().to_string(),
//                     disk.avail.to_owned(),
//                     " | ".to_string(),
//                     "Mountpoint: ".purple().to_string(),
//                     disk.mount.to_owned(),
//                     "\n\n".to_string(),
//                 ];
//                 for thing in diskprints {
//                     print!("{thing}");
//                 }
//             }
//         }
//         None => {}
//     }
// }
