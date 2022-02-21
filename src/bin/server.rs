use std::{
    io::{BufReader, Write},
    net::TcpListener,
};

use colored::*;
use log::debug;

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
            eprintln!("Failed to bind to {}:{} : {}", IP_ADDR, PORT, err);
            std::process::exit(1);
        }
    };

    loop {
        let (stream, addr) = match listener.accept() {
            Ok(stream) => stream,
            Err(err) => {
                eprintln!("Failed to accept connection: {}", err);
                continue;
            }
        };
        debug!("Accepted connection to {}", addr);
        let mut stream = BufReader::new(stream);
    }
}
