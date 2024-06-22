use std::net::TcpStream;
use crate::cli_args_parser::Arguments;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::io::{self, Write};
use std::sync::Arc;

const MAX_PORTS: u16 = 65535;

pub fn scan_ports(scan_args: Arguments) -> Vec<u16> {
    let mut open_ports: Vec<u16> = Vec::new();
    let num_threads = scan_args.num_threads;

    let (tx, rx) = channel::<u16>();

    let scan_args_arc = Arc::new(scan_args);

    for i in 0..num_threads {
        let thread_tx = tx.clone();
        let args = Arc::clone(&scan_args_arc);
        let start_port = (MAX_PORTS / num_threads) * i;
        let end_port = (MAX_PORTS / num_threads) * (i + 1);

        thread::spawn(move || {
                scan(thread_tx, &args, start_port, end_port);
        });
    }

    drop(tx); // Drop the original sender to close the channel

    for p in rx {
        open_ports.push(p);
    }

    open_ports.sort();
    open_ports
}

fn scan(tx: Sender<u16>, scan_args: &Arc<Arguments>, start_port: u16, end_port: u16) {
    let start_port = start_port + 1;
    for port in start_port..end_port {
        match TcpStream::connect((scan_args.ip_addr.clone(), port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).expect("Unable to send data back to parent thread");
            }
            Err(_) => {}
        }
    }
}
