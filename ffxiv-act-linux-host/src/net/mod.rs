use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};

use std::sync::mpsc;

use std::thread;
use std::thread::JoinHandle;

use crate::pcap;
use pcap::Device;

use etherparse::SlicedPacket;

use crate::NetConfig;

pub fn start_packet_redirection(net_config: NetConfig) {
    let interface = net_config.interface;
    let host_exclude = net_config.hostname_exclude;
    let sender = start_incoming_sync_host();
    let device = Device::list().unwrap().into_iter().filter(|d| d.name == interface).next().unwrap();
    println!("Capturing on {}", device.name);
    let mut cap = device.open().unwrap();
    cap.filter(format!("(src port 54992) && (src host not {})", host_exclude).as_str()).expect("Unable to apply filters");

    println!("Setup pcap for network redirection");
    loop {
        let p = cap.next().unwrap();

        let data = p.data.to_vec();
        let pa = SlicedPacket::from_ethernet(data.as_slice()).unwrap();

        let pref = pa.payload;

        if pref.len() == 0 {
            continue;
        } else {
            sender.send(pref.to_vec()).expect("Unable to send message to host thread");
        }

    }

}

fn start_incoming_sync_host() -> mpsc::Sender<Vec<u8>> {
    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    thread::spawn(move || {
        let tcp = TcpListener::bind("0.0.0.0:54992").expect("Unable to bind listener");
        println!("Opened fake ffxiv server on 0.0.0.0:54992");
        let (mut inc,_) = tcp.accept().expect("Unable to accept connection");
        let mut iter = rx.try_iter();
        while let Some(_) = iter.next() {

        }
        for data in rx {
            println!("sending packet of size {} to client", data.len());
            inc.write(&data[..]).expect("Unable to send packet");
        }

    });
    tx

}