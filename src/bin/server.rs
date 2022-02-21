use std::{
    collections::HashMap,
    error::Error,
    io::{BufRead, Write},
    net::TcpListener,
    path::Path,
};

use colored::*;
use csv::ReaderBuilder;
use log::{debug, error, info, warn};
use mini_ftp::{BufTcpStream, Command};
use serde::Deserialize;

const IP_ADDR: &str = "127.0.0.1";
const PORT: u16 = 25000;

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

    let info = if let Some(user_filename) = std::env::args().next() {
        match parse_user_info(user_filename) {
            Ok(info) => info,
            Err(err) => {
                error!("Failed to read user info file: {}", err);
                std::process::exit(1);
            }
        }
    } else {
        error!("Expected: <user-filename>");
        std::process::exit(1);
    };

    let listener = match TcpListener::bind((IP_ADDR, PORT)) {
        Ok(listener) => listener,
        Err(err) => {
            error!("Failed to bind to {}:{} : {}", IP_ADDR, PORT, err);
            std::process::exit(1);
        }
    };

    loop {
        let (stream, addr) = match listener.accept() {
            Ok(stream) => stream,
            Err(err) => {
                warn!("Failed to accept connection: {}", err);
                continue;
            }
        };
        debug!("Accepted connection to {}", addr);
        let stream = BufTcpStream::new(stream);
        let mut ctx = Context::new(stream, &info);

        // Now we have a connection
        // Listen for commands, quit if connection closed
        loop {
            let mut buf = Vec::new();
            if let Err(err) = ctx.stream.read_until(b'\0', &mut buf) {
                warn!("Error while reading: {}", err);
                break;
            }
            if buf.is_empty() {
                debug!("Done with {}", addr);
                break;
            }
            let cmd = match Command::from_bytes(&buf) {
                Ok(cmd) => cmd,
                Err(err) => {
                    info!("Invalid command: {}", err);
                    continue;
                }
            };

            match &cmd {
                Command::User(name) => {
                    if !ctx.handle_user(name) {
                        break;
                    }
                }
                Command::Password(_) => todo!(),
                Command::Cd(_) => todo!(),
                Command::Dir => todo!(),
                Command::Get(_, _) => todo!(),
                Command::Put(_, _) => todo!(),
                Command::Mget(_) => todo!(),
                Command::Mput(_) => todo!(),
                Command::Open(_, _) => unreachable!("Server cannot do open"),
                Command::Quit => unreachable!("Server cannot do quit"),
                Command::Lcd(_) => unreachable!("Server cannot do lcd"),
            }
        }
    }
}

#[derive(Debug)]
struct Context<'info> {
    stream: BufTcpStream,
    info: &'info HashMap<String, UserInfo>,
    expected_pass: Option<&'info str>,
    state: ContextState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContextState {
    Connected,
    UserSent,
    Ready,
}

impl<'info> Context<'info> {
    fn new(stream: BufTcpStream, info: &'info HashMap<String, UserInfo>) -> Self {
        Self {
            stream,
            info,
            expected_pass: None,
            state: ContextState::Connected,
        }
    }

    fn handle_user(&mut self, name: &str) -> bool {
        if self.state != ContextState::Connected {
            if let Err(err) = self.stream.write_all(b"600\0") {
                info!("{}", err);
                return false;
            }
            return true;
        }
        if let Some(user_info) = self.info.get(name) {
            self.state = ContextState::UserSent;
            self.expected_pass = Some(&user_info.pass);
            if let Err(err) = self.stream.write_all(b"200\0") {
                info!("{}", err);
                return false;
            }
        } else if let Err(err) = self.stream.write_all(b"500\0") {
            info!("{}", err);
            return false;
        }
        true
    }
}

#[derive(Debug, Deserialize)]
struct UserInfo {
    name: String,
    pass: String,
}

fn parse_user_info<P: AsRef<Path>>(path: P) -> Result<HashMap<String, UserInfo>, Box<dyn Error>> {
    let mut info = HashMap::new();
    let mut rdr = ReaderBuilder::new().has_headers(false).from_path(path)?;
    for row in rdr.deserialize() {
        let user_info: UserInfo = row?;
        info.insert(user_info.name.clone(), user_info);
    }
    Ok(info)
}
