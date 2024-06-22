mod cli_args_parser;
use cli_args_parser::Arguments;

use std::env;
use std::str::FromStr;
use std::process;



fn main() {

    let cli_args : Vec<String> = env::args().collect();

    match Arguments::new(&cli_args){
        Ok(args) => {
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
