use std::{
    io::{Read, Write},
    net::{Ipv4Addr, TcpStream},
    str::FromStr,
};

use colored::*;

use log::debug;
use mini_ftp::{BufTcpStream, Command};

macro_rules! client_err {
    ($format_string: expr, $($arg:tt)*) => (eprintln!("{}", format!($format_string, $($arg)*).red()));
    ($string:expr) => (eprintln!("{}", format!("{}", $string).red()));
}

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
                        client_err!(err);
                        continue;
                    }
                };
                if matches!(cmd, Command::Quit) {
                    println!("Quit");
                    return;
                }
                debug!("{:?}", cmd);
                match &cmd {
                    Command::Open(addr, port) => ctxt.handle_open(*addr, *port),
                    Command::User(_) => ctxt.handle_user(&cmd),
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
                        client_err!("Failed to readline: {}", err);
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
    conn: Option<BufTcpStream>,
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
            client_err!("Already opened");
            return;
        }
        let stream = match TcpStream::connect((addr, port)) {
            Ok(stream) => stream,
            Err(err) => {
                client_err!("Failed to open: {}", err);
                return;
            }
        };
        self.conn = Some(BufTcpStream::new(stream));
        self.conn_stat = ConnectionStatus::Connected;
    }

    fn handle_user(&mut self, cmd: &Command) {
        if self.conn_stat != ConnectionStatus::Connected {
            client_err!("Cannot set user now");
            return;
        }
        let conn = self.conn.as_mut().unwrap();
        let cmd_bytes = cmd.to_bytes();
        conn.write_all(&cmd_bytes).ok();
        let mut reply = [0_u8; 4];
        if let Err(err) = conn.read_exact(&mut reply) {
            client_err!("Failed to read: {}", err);
            return;
        }
        if &reply == b"600\0" {
            client_err!("Cannot set user now");
        } else if &reply == b"500\0" {
            client_err!("User doesn't exist");
        } else if &reply == b"200\0" {
            self.conn_stat = ConnectionStatus::SentUser;
        } else {
            client_err!("Invalid response from server");
        }
    }
}
