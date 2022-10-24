use std::error::Error;
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread::sleep;

const ASCII_ETX: u8 = 0x03;


pub fn get_stream() -> TcpStream {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    let stream = TcpStream::connect("127.0.0.1:8081").unwrap();
    let _ = listener.incoming();
    stream
}

impl<T: Read> MyRead for T {
    fn read_all_string(&mut self) -> Result<String, Box<dyn Error>> {
        // Wrap the stream in a BufReader, so we can use the BufRead methods
        let mut reader = BufReader::new(self);
        // Read current current data in the TcpStreamkik
        let received = String::from_utf8(reader.fill_buf()?.to_vec()).unwrap();
        reader.consume(received.len());
        Ok(received)
    }


    fn read_all_vec(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        // Wrap the stream in a BufReader, so we can use the BufRead methods
        let mut reader = BufReader::new(self);
        // Read current current data in the TcpStreamkik
        let mut vec = Vec::new();
        reader.read_to_end(&mut vec).expect("TODO: panic message");
        // let received = reader.fill_buf()?.to_vec();
        // reader.consume(received.len());
        Ok(vec)
    }

    fn read_string(&mut self) -> Result<String, Box<dyn Error>> {
        const LEN: usize = 1024;
        let mut buffer = [0; LEN];
        let mut input = String::new();

        loop {
            let n = self.read(&mut buffer)?;
            let s = str::from_utf8(&buffer[0..n])?;
            input.push_str(s);
            if n != LEN { break; }
        }
        Ok(input)
    }
}

pub trait MyRead {
    fn read_all_string(&mut self) -> Result<String, Box<dyn Error>>;
    fn read_all_vec(&mut self) -> Result<Vec<u8>, Box<dyn Error>>;
    fn read_string(&mut self) -> Result<String, Box<dyn Error>>;
}