mod cli_args_parser;
use cli_args_parser::Arguments;

mod port_scanner;
use port_scanner::scan_ports;

use std::env;
use std::str::FromStr;
use std::process;

use std::sync::mpsc::{Sender, channel};
use std::thread;

fn main() {

    let cli_args : Vec<String> = env::args().collect();

    match Arguments::new(&cli_args){
        Ok(args) => {
            let open_ports = scan_ports(args);
            for open_port in open_ports{
                println!("{}", open_port);
            }
            process::exit(0);
        },
        Err(err) => {
            if err == "no_error" {
                process::exit(0)
            }
            eprintln!("Error: {}", err)
        },
        _ => {process::exit(0)}
    }


}
