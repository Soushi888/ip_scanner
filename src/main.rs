use std::env;
use std::io;
use std::io::Write;
use std::net::{AddrParseError, IpAddr, TcpStream};
use std::num::ParseIntError;
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

const MAX: u16 = 65535;

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
        return if let Ok(ipaddr) = IpAddr::from_str(&f) {
            Ok(Arguments { flag: String::from(""), ipaddr, threads: 4 })
        } else {
            let flag = f;

            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want
                \r\t -h or -help to show this help message");
                Err("help")
            } else if flag.contains("-h") || flag.contains("-help") {
                Err("Too many arguments")
            } else if flag.contains("-j") {
                let threads = match args[2].parse::<u16>() {
                    Ok(c) => c,
                    Err(_) => return Err("failed to parse thread number")
                };
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(c) => c,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6")
                };
                Ok(Arguments { threads, flag, ipaddr })
            } else {
                Err("invalid syntax")
            }
        };
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads { break; }
        port += num_threads;
    }
}

fn main() {
    let argus: Vec<String> = env::args().collect();
    let program = &argus[0];
    let arguments = Arguments::new(&argus).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit((0));
            }
        }
    );

    let num_threads = arguments.threads;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!();
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
