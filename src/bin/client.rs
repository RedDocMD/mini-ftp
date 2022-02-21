use std::{
    io::{BufReader, Write},
    net::{Ipv4Addr, TcpStream},
    str::FromStr,
};

use colored::*;

use log::debug;
use mini_ftp::Command;

fn main() {
    env_logger::builder()
        .format(|buf, rec| {
            let line = rec
                .line()
                .map_or(String::new(), |line| format!(":{}", line));
            let file = rec
                .file()
                .map_or(String::new(), |file| format!(" {}", file));
            let prelude = format!("[{}{}{}]", rec.level(), file, line);
            writeln!(buf, "{} {}", prelude.cyan(), rec.args())
        })
        .write_style(env_logger::WriteStyle::Always)
        .init();

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
                debug!("{:?}", cmd);
                match cmd {
                    Command::Open(addr, port) => ctxt.handle_open(addr, port),
                    Command::User(name) => ctxt.handle_user(name),
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ConnectionStatus {
    Disconnnected,
    Connected,
    SentUser,
    Ready,
}

#[derive(Debug)]
struct Context {
    conn: Option<BufReader<TcpStream>>,
    conn_stat: ConnectionStatus,
}

impl Context {
    fn new() -> Self {
        Self {
            conn: None,
            conn_stat: ConnectionStatus::Disconnnected,
        }
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
        self.conn_stat = ConnectionStatus::Connected;
    }

    fn handle_user(&mut self, name: String) {
        if self.conn_stat != ConnectionStatus::Connected {
            eprintln!("{}", "Cannot set user now".red());
            return;
        }
    }
}
