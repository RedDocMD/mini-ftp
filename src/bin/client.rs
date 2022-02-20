use std::str::FromStr;

use mini_ftp::Command;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
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
                println!("{:?}", cmd);
            }
            Err(err) => {
                eprintln!("Failed to readline: {}", err);
                std::process::exit(1);
            }
        }
    }
}
