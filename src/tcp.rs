use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};

#[derive(Debug)]
pub struct BufTcpStream {
    stream: BufReader<TcpStream>,
}

impl BufTcpStream {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: BufReader::new(stream),
        }
    }
}

impl BufRead for BufTcpStream {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.stream.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.stream.consume(amt);
    }
}

impl Read for BufTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}

impl Write for BufTcpStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stream.get_mut().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.get_mut().flush()
    }
}
