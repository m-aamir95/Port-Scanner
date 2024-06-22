use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug)]
pub struct Arguments {
    pub ip_addr: IpAddr,
    pub num_threads : u16,
    pub flag : String
}

impl Arguments{
    pub fn new(args : &[String]) -> Result<Arguments, String>{

        if args.len() < 2 { //At least has to have, ["program_name", "192.168.8.1"]
            return Err("too few arguments, IP address is mandatory".parse().unwrap());

        }else if args.len() > 4 {//At max, can't have more than the following arguments.
            //["program_name", "-j|-h", "4", "192.168.8.1"

            return Err("too many arguments please see help e.g, -h".parse().unwrap());
        }

        // Check if the first token is an IP address
        let token = args[1].as_str();

        if let Ok(ip) = IpAddr::from_str(token){
            return Ok(Arguments{ip_addr: ip, num_threads: 1000, flag : String::from(" ") });
        }else{

            //Maybe the token is -j to specify the number of jobs and not the IP
            //Or maybe it's for help
            match token {
                "-j"|"--jobs" if args.len() == 4 => {

                    //Next token will hold num threads
                    let num_threads = &args[2];
                    if let Ok(n_threads) = num_threads.parse::<u16>() {

                        //Finally parse the ip address
                        if let Ok(ip_addr) = IpAddr::from_str(args[3].as_str()){
                            return Ok(Arguments{ip_addr, num_threads : n_threads, flag : String::from(" ")})
                        }else {
                            Err("Not a valid IP address".parse().unwrap())
                        }
                    }else{
                        Err("Not a valid num jobs/threads".parse().unwrap())
                    }
                },
                "-j|--jobs" => {
                    Err("Please specify Number of Jobs and IP address".parse().unwrap())
                },
                "-h"|"--help" if args.len() == 2 => {
                    println!("Usage: -j or --jobs to specify number of threads, and -h or --help to get help");
                    Err("no_error".parse().unwrap())
                },
                _ => {
                    println!("Usage: -j or --jobs to specify number of threads, and -h or --help to get help");
                    Err("no_error".parse().unwrap())
                }
            }
        }
    }
}