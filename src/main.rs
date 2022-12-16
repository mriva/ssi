#![allow(unused)]
mod config;

use std::{fs::{File, self}, io::{BufReader, Result, BufRead}, process::exit, str::Lines};
use regex::Regex;
use config::*;

fn main() {
    let hosts = get_config_lines()
        .map(extract_hosts);

    if let Err(error) = hosts {
        println!("Errore");
        exit(exitcode::DATAERR);
    }

    for host in hosts.unwrap().iter() {
        println!("{}", host);
    }

    exit(exitcode::OK);
}

#[cfg(test)]

mod tests {
    use pretty_assertions::{assert_eq, assert_ne};
    use super::*;

    #[test]
    fn can_extract_hosts() {
        let lines = vec![
            String::from("Host test-one"),
            String::from(" HostName 1.2.3.4"),
        ];

        let hosts = extract_hosts(lines);

        assert_eq!(vec!["test-one"], hosts);
    }

    #[test]
    fn will_not_consider_hosts_without_hostname() {
        let lines = vec![
            String::from("Host test-one"),
            String::from("Host test-two"),
            String::from("Host test-three"),
            String::from("Host test-four"),
        ];

        let hosts = extract_hosts(lines);
        let expected: Vec<String> = Vec::new();

        assert_eq!(expected, hosts);
    }
}
