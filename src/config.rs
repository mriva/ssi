use std::{fs::{File, self}, io::{BufReader, Result, BufRead}, process::exit, str::Lines};
use dirs::home_dir;
use regex::Regex;

pub fn get_config_lines() -> Result<Vec<String>> {
    let home_str = home_dir()
        .map(|x| x.to_str().unwrap().to_string())
        .unwrap();
    let config_path = format!("{}/.ssh/config", home_str);
    let config = fs::read_to_string(config_path)?;
    let lines: Vec<String> = config.lines().map(|i| i.to_string()).collect();

    Ok(lines)
}

pub fn extract_hosts(lines: Vec<String>) -> Vec<String> {
    let mut current_host = String::new();
    let mut hosts: Vec<String> = Vec::new();
    let re = Regex::new(r"^Host (\S+)").unwrap();

    for line in lines {
        if line.contains("Host") {
            let hostname = re.captures(&line);
            match hostname {
                Some(cap) => current_host = String::from(cap.get(1).unwrap().as_str()),
                None => (),
            }
        }

        if line.contains("HostName") {
            hosts.push(current_host);
            current_host = String::new();
            continue;
        }
    }

    hosts
}

