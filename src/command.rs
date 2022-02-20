use std::{net::Ipv4Addr, str::FromStr};

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::space0,
    combinator::map_res,
    multi::{count, separated_list1},
    sequence::preceded,
    IResult,
};

use crate::error::MiniFtpError;

#[derive(Debug)]
pub enum Command {
    Open(Ipv4Addr, u16),
    User(String),
    Password(String),
    Cd(String),
    Lcd(String),
    Dir,
    Get(String, String),
    Put(String, String),
    Mget(Vec<String>),
    Mput(Vec<String>),
    Quit,
}

impl FromStr for Command {
    type Err = MiniFtpError;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        let trimmed_inp = inp.trim();
        let (res, cmd) = match parse_command(trimmed_inp) {
            Ok(ans) => ans,
            Err(err) => return Err(MiniFtpError::CommandParseError(err.to_string())),
        };
        if !res.is_empty() {
            return Err(MiniFtpError::CommandParseError(String::from(
                "trailing characters",
            )));
        }
        Ok(cmd)
    }
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    use nom::{
        error::{Error, ErrorKind},
        Err as NomErr,
    };

    let (input, cmd_str) = parse_str(input)?;
    match cmd_str {
        "open" => {
            let (input, ip) = map_res(parse_str, Ipv4Addr::from_str)(input)?;
            let (input, port) = map_res(parse_str, u16::from_str)(input)?;
            Ok((input, Command::Open(ip, port)))
        }
        "user" => {
            let (input, name) = parse_str(input)?;
            Ok((input, Command::User(name.into())))
        }
        "pass" => {
            let (input, pass) = parse_str(input)?;
            Ok((input, Command::Password(pass.into())))
        }
        "cd" => {
            let (input, dir) = parse_str(input)?;
            Ok((input, Command::Cd(dir.into())))
        }
        "lcd" => {
            let (input, dir) = parse_str(input)?;
            Ok((input, Command::Lcd(dir.into())))
        }
        "dir" => Ok((input, Command::Dir)),
        "get" => {
            let (input, names) = count(parse_str, 2)(input)?;
            Ok((input, Command::Get(names[0].into(), names[1].into())))
        }
        "put" => {
            let (input, names) = count(parse_str, 2)(input)?;
            Ok((input, Command::Put(names[0].into(), names[1].into())))
        }
        "mget" => {
            let (input, names) = separated_list1(tag(","), parse_str)(input)?;
            Ok((
                input,
                Command::Mget(names.into_iter().map(String::from).collect()),
            ))
        }
        "mput" => {
            let (input, names) = separated_list1(tag(","), parse_str)(input)?;
            Ok((
                input,
                Command::Mput(names.into_iter().map(String::from).collect()),
            ))
        }
        "quit" => Ok((input, Command::Quit)),
        _ => Err(NomErr::Failure(Error {
            input,
            code: ErrorKind::Fail,
        })),
    }
}

fn is_space(ch: char) -> bool {
    ch == ' '
}

fn parse_str(input: &str) -> IResult<&str, &str> {
    preceded(space0, take_till(is_space))(input)
}
