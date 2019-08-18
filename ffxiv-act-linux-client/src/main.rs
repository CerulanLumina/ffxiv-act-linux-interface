use std::time::Duration;

use std::net::{TcpStream, TcpListener};
use std::fs::File;
use std::io::prelude::*;
use std::thread;

use serde_json::from_reader;
use serde::Deserialize;

use byteorder::{ByteOrder, LittleEndian};

use std::net::ToSocketAddrs;

mod models;

use models::*;
//
//static mut SIGNATURE: [u8; 156] = [
//    // ZoneID
//    0xf3,0x0f,0x10,0x8d,0x08,0x04,0x00,0x00,0x4c,0x8d,0x85,0x58,0x06,0x00,0x00,0x0f,0xb7,0x05,
//    0x00, 0x00, 0x00, 0x00,
//    // Target
//    0x41,0xbc,0x00,0x00,0x00,0xe0,0x41,0xbd,0x01,0x00,0x00,0x00,0x49,0x3b,0xc4,0x75,0x55,0x48,0x8d,0x0d,
//    0x04,0x00,0x00,0x00,
//    // ChatLog
//    0xe8,0x00,0x00,0x00,0x00,0x85,0xc0,0x74,0x0e,0x48,0x8b,0x0d,0x00,0x00,0x00,0x00,0x33,0xD2,0xE8,0x00,0x00,0x00,0x00,0x48,0x8b,0x0d,
//    0x04,0x00,0x00,0x00,
//    // MobArray
//    0x48,0x8b,0x42,0x08,0x48,0xc1,0xe8,0x03,0x3d,0xa7,0x01,0x00,0x00,0x77,0x24,0x8b,0xc0,0x48,0x8d,0x0d,
//    0x04,0x00,0x00,0x00,
//    // PartyList
//    0x48,0x8D,0x7C,0x24,0x20,0x66,0x66,0x0F,0x1F,0x84,0x00,0x00,0x00,0x00,0x00,0x48,0x8B,0x17,0x48,0x8D,0x0D,
//    0x04,0x00,0x00,0x00,
//    // ServerTime
//    0x0f,0xb7,0xc0,0x89,0x47,0x10,0x48,0x8b,0x0d,
//    0x04,0x00,0x00,0x00,
//    // Player
//    0x83,0xf9,0xff,0x74,0x12,0x44,0x8b,0x04,0x8e,0x8b,0xd3,0x48,0x8d,0x0d,
//    0x04,0x00,0x00,0x00,
//];

static mut ALL_MEMORY: AllMemory = AllMemory::create();

//const ZONE_PTR_INDEX: usize = 156;
//const TARGET_PTR_INDEX: usize = 42;
//const CHATLOG_PTR_INDEX: usize = 72;
//const MOBARRAY_PTR_INDEX: usize = 96;
//const PARYLIST_PTR_INDEX: usize = 121;
//const SERVERTIME_PTR_INDEX: usize = 134;
//const PLAYER_PTR_INDEX: usize = 152;



fn main() {
    unsafe { setup_memory(); }

    let config: Config = {
        let mut file = File::open("config.json").expect("Unable to open config file");
        from_reader(&mut file).expect("Unable to read / parse config file")
    };
    let addr2 = config.address.clone();
    thread::spawn(move || {
        let mut addr = addr2.to_socket_addrs().unwrap().next().unwrap();
        addr.set_port(54992);
        let mut tcp_ffxiv = TcpStream::connect(addr).unwrap();
        let mut byte_buffer_ffxiv = [0u8; 32768];
        loop {
            let read = tcp_ffxiv.read(&mut byte_buffer_ffxiv).unwrap();
            println!("read {} bytes", read);
        }
    });
    let mut tcpstream = TcpStream::connect(config.address).expect("Unable to connect to server");
    let mut byte_buffer = [0u8; 1];
    loop {
        match tcpstream.read(&mut byte_buffer) {
            Ok(bytes) => {
                if bytes == 1 {
                    let packet_type = byte_buffer[0];
                    if packet_type == 0x01 {
                        // Zone packet
                        let mut zone_buffer = [0u8; 4];
                        tcpstream.read_exact(&mut zone_buffer).expect("malformed packet");
                        let zone = LittleEndian::read_u32(&zone_buffer);
                        println!("new zone: {}", zone);
                        unsafe { set_zone(zone); }
                    }
                }
            },
            Err(_) => break
        }
    }

    unsafe {
        println!("Memory sync bank ptr: {:p}", &ALL_MEMORY as *const AllMemory)
    }

}

#[derive(Deserialize)]
struct Config {
    pub address: String,
}

unsafe fn setup_memory() {
    SERVER_2.ptr3 = (&SERVER_3) as *const ServerTimePart3 as u64;
    SERVER_1.ptr2 = (&SERVER_2) as *const ServerTimePart2 as u64;
    ALL_MEMORY.server_time.ptr = (&SERVER_1) as *const ServerTimePart1 as u64;
}

unsafe fn set_zone(zone: u32) {
    ALL_MEMORY.zone_id.data = zone;
}