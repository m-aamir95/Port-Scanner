

use std::env;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug)]
struct Arguments {
    ip_addr: IpAddr,
    num_threads : u16,
    flag : String
}

impl Arguments{
    fn new(args : &[String]) -> Result<Arguments, String>{

    if args.len() < 2 {
        return Err("too few arguments, IP address is mandatory".parse().unwrap());
    }else if args.len() > 4 {
        return Err("too many arguments please see help e.g, -h".parse().unwrap());
    }

    // Check if the first token is an IP address
    let token = args[1].as_str();
        if let Ok(ip) = IpAddr::from_str(token){
            return Ok(Arguments{ip_addr: ip, num_threads: 4, flag : String::from(" ") });
        }else{

            //Maybe the token is -j to specify the number of jobs and not the IP
            //Or maybe its for help
            match token {
                "-j"|"--jobs" if args.len() == 4=> {

                    //Next token will hold num threads
                    let num_threads = &args[2];
                    if let Ok(n_threads) = num_threads.parse::<u16>() {

                        //Finally parse the ip address
                        if let Ok(ip_addr) = IpAddr::from_str(args[3].as_str()){
                            return Ok(Arguments{ip_addr : ip_addr, num_threads : n_threads, flag : String::from(" ")})
                        }else {
                            Err("Not a valid IP address".parse().unwrap())
                        }
                    }else{
                        Err("Not a valid num jobs/threads".parse().unwrap())
                    }
                }
                "-h"|"--help" if args.len() == 2 => {
                    println!("Usage: -j or --jobs to specify number of threads, and -h or --help to get help");
                    Err(" ".parse().unwrap())
                }
                _ => {
                    println!("Usage: -j or --jobs to specify number of threads, and -h or --help to get help");
                    Err(" ".parse().unwrap())
                }
            }
        }
    }
}

fn main() {

    let cli_args : Vec<String> = env::args().collect();
    let parsed_args = Arguments::new(&cli_args);

    println!("{:?}", parsed_args);
}
