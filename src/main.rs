use std::env;
use std::net::{AddrParseError, IpAddr};
use std::num::ParseIntError;
use std::str::FromStr;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let f = args[1].to_owned();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments { flag: String::from(""), ipaddr, threads: 4 });
        } else {
            let flag = f;
            println!("{}", f);

            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want
                \r\n      -h or -help to show this help message");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many arguments");
            } else if flag.contains("-j") {
                let threads = match args[2].parse::<u16>() {
                    Ok(c) => c,
                    Err(_) => return Err("failed to parse thread number")
                };
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(c) => c,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6")
                };
                return Ok(Arguments { threads, flag, ipaddr });
            } else {
                return Err("invalid syntax");
            }
        }
    }
}

fn main() {
    let argus: Vec<String> = env::args().collect();
    let program = &argus[0];

    println!("{:?}", argus);
}

// TODO: help screen
// ip_sniffer.exe -h

// TODO: set number of threads
// ip_sniffer.exe -j 100 192.168.1.1

// bind an IP address
// ip_sniffer.exe 192.168.1.1
