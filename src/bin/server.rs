use std::{
    io::{BufRead, Write},
    net::TcpListener,
};

use colored::*;
use log::{debug, error, info, warn};
use mini_ftp::{BufTcpStream, Command};

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
        let mut ctx = Context::new(stream);

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
                Command::User(_) => ctx.handle_user(&cmd),
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
struct Context {
    stream: BufTcpStream,
}

impl Context {
    fn new(stream: BufTcpStream) -> Self {
        Self { stream }
    }

    fn handle_user(&mut self, cmd: &Command) {
        todo!("Handle user!");
    }
}
