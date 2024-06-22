use std::net::TcpStream;
use crate::cli_args_parser::Arguments;

use std::thread;
use std::sync::mpsc::{Sender, channel};

const max_ports : u16 = 65535;
pub fn scan_ports(scan_args: &Arguments) -> Vec<String> {

    let mut open_ports : Vec<String> = Vec::new();
    // let (tx, rx) = channel();

    //
    // for i in 0..scan_args.num_threads{
    //     thread::spawn(move || {
    //
    //     });
    // }
    for port in 1..=max_ports{
        match TcpStream::connect((scan_args.ip_addr, port)

        ) {
            Ok(_) => {open_ports.push(format!("Port # {} is open at {} address", port, scan_args.ip_addr ));}
            Err(_) => {}
        }
    }

    open_ports
}