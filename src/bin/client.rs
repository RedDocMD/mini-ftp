use std::{
    io::BufReader,
    net::{Ipv4Addr, TcpStream},
    str::FromStr,
};

use colored::*;

use mini_ftp::Command;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    let mut ctxt = Context::new();
    loop {
        match rl.readline("ftp> ") {
            Ok(line) => {
                let cmd = match Command::from_str(&line) {
                    Ok(cmd) => cmd,
                    Err(err) => {
                        eprintln!("{}", err);
                        continue;
                    }
                };
                if matches!(cmd, Command::Quit) {
                    println!("Quit");
                    return;
                }
                eprintln!("{:?}", cmd);
                match cmd {
                    Command::Open(addr, port) => ctxt.handle_open(addr, port),
                    Command::User(_) => todo!(),
                    Command::Password(_) => todo!(),
                    Command::Cd(_) => todo!(),
                    Command::Lcd(_) => todo!(),
                    Command::Dir => todo!(),
                    Command::Get(_, _) => todo!(),
                    Command::Put(_, _) => todo!(),
                    Command::Mget(_) => todo!(),
                    Command::Mput(_) => todo!(),
                    Command::Quit => unreachable!("Already handled above"),
                }
            }
            Err(err) => {
                use rustyline::error::ReadlineError::*;
                match err {
                    Eof => {
                        println!("Quit");
                        return;
                    }
                    Interrupted => {
                        println!("Interrupted");
                        return;
                    }
                    _ => {
                        eprintln!("Failed to readline: {}", err);
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}

struct Context {
    conn: Option<BufReader<TcpStream>>,
}

impl Context {
    fn new() -> Self {
        Self { conn: None }
    }

    fn handle_open(&mut self, addr: Ipv4Addr, port: u16) {
        if self.conn.is_some() {
            eprintln!("{}", "Already opened".red());
            return;
        }
        let stream = match TcpStream::connect((addr, port)) {
            Ok(stream) => stream,
            Err(err) => {
                eprintln!("{}", format!("Failed to open: {}", err).red());
                return;
            }
        };
        self.conn = Some(BufReader::new(stream));
    }
}
