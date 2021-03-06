//mod signatures;
mod utils;
mod mem;
mod net;

extern crate byteorder;
extern crate read_process_memory;
extern crate proc_maps;
extern crate serde;
extern crate serde_json;
extern crate hex;
extern crate pcap;
extern crate etherparse;
extern crate bincode;
extern crate flate2;


use serde::{Deserialize};
use std::fs::File;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use crate::net::get_src_port;

fn main() {
    let config_fixed: Config = {
        if let Ok(file) = File::open("config.json") {
            if let Ok(config) = serde_json::from_reader(file) {
                config
            } else {
                eprintln!("Couldn't read / parse config file.");
                std::process::exit(1);
            }
        } else {
            eprintln!("Couldn't open config file.");
            std::process::exit(1);
        }
    };

    loop {
        let config = config_fixed.clone();
        let ffxiv = wait_for_ffxiv();

        let (tx, rx) = mpsc::channel();
        let mem_tx = tx.clone();
        let net_tx = tx;

        let mem_config = config.mem_config;
        let net_config = config.net_config;

        // Memory
        thread::spawn(move || {
            if !mem::begin(ffxiv, mem_config) {
                mem_tx.send(false).unwrap();
            }
            mem_tx.send(true).unwrap();
        });

        // Network
        thread::spawn(move || {
            if !net::start_packet_redirection(net_config, ffxiv) {
                net_tx.send(false).unwrap();
            }
            net_tx.send(true).unwrap();
        });

        for rec in rx {
            if !rec {
                eprintln!("Terminating due to error in memory or network thread.");
                std::process::exit(1);
            }
        }

    }

}

fn wait_for_ffxiv() -> i32 {
    let mut ffxiv;
    let mut port;
    let mut said_message = false;
    let mut said_ffxiv = false;
    let mut said_port = false;
    loop {
        ffxiv = utils::find_ffxiv();
        if ffxiv.is_some() {
            if !said_ffxiv {
                println!("Found FFXIV on PID {}!", ffxiv.unwrap());
                said_ffxiv = true;
            }
            port = get_src_port(ffxiv.unwrap());
            if port.is_some() {
                println!("Found FFXIV network port at {}!", port.unwrap());
                println!("Starting memory-sync and network-passthrough now...");
                break;
            } else {
                if !said_port {
                    println!("Waiting for FFXIV network connection...");
                    said_port = true;
                }
                std::thread::sleep(Duration::from_secs(1));
            }
            said_message = false;
        } else {
            if !said_message {
                println!("Waiting for FFXIV...");
                said_message = true;
            }
            said_ffxiv = false;
            std::thread::sleep(Duration::from_secs(1));
        }
    }
    ffxiv.unwrap()
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub net_config: NetConfig,
    pub mem_config: MemConfig,
}

#[derive(Deserialize, Clone)]
pub struct NetConfig {
    pub interface: String,
    pub hostname_exclude: String,
    pub bind_address: String,
}

#[derive(Deserialize, Clone)]
pub struct MemConfig {
    pub bind_address: String,
}
