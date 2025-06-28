use std::{env, process::exit};
use reqwest::blocking::Client;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Incorrect usage. Exiting..");
        exit(1)
    }

    let newmac = format_mac_address(args[1].as_str());
    if let Ok(mac_variants) = newmac {
        let response = get_mac(mac_variants[0].as_str());
        if response.is_ok() {
            let vendor = response.unwrap().text().unwrap();
            println!("{vendor}");
            for mac in mac_variants {
                println!("{mac}")
            }
        }
    }
    }

fn get_mac(mac: &str) -> Result<reqwest::blocking::Response, reqwest::Error>{
    const URL: &str = "https://api.macvendors.com/";
    let query_url = format!("{}{}", URL, mac);
    let http_client = Client::new();
    http_client.get(query_url).send()

}

fn format_mac_address(mac: &str) -> Result<Vec<String>, String> {
    if !is_mac_valid(&mac) {
        return Err(format!("invalid MAC address firnat: {}", mac));
    }

    let seperators = [":", "-", "."];
    let cleaned_mac: String = mac.chars().filter(|c| *c != ':').collect();
    let chunks = split_every(&cleaned_mac, 2);
    let result: Vec<String> = seperators.iter().map(|sep| chunks.join(sep)).collect();

    Ok(result)
}

fn is_mac_valid(mac: &str) -> bool {
    let re = Regex::new(r"^([0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}$");
    match re {
        Ok(regex) => regex.is_match(mac),
        Err(e) => {
            eprintln!("Regex compilation error: {e}");
            false
        },
    }
}

fn split_every(s: &str, chunk_size: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut i = 0;

    while i < s.len() {
        let end = (i + chunk_size).min(s.len());

        if let Some(valid_chunk) = s.get(i..end) {
            chunks.push(valid_chunk.to_string());
        } else {
            let chunk: String = s.chars().skip(i).take(chunk_size).collect();
            chunks.push(chunk);
            break;
        }
        i += chunk_size;
    }
    chunks
}