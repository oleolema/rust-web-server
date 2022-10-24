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
    // fn read_all_string(&mut self) -> Result<String, Box<dyn Error>> {
    //     const LEN: usize = 2;
    //     let mut buffer = [0; LEN];
    //     let mut input = String::new();
    //
    //     loop {
    //         let n = self.read(&mut buffer)?;
    //         let s = str::from_utf8(&buffer[0..n])?;
    //         input.push_str(s);
    //         if n != LEN { break; }
    //     }
    //     Ok(input)
    // }
    fn read_all_string(&mut self) -> Result<String, Box<dyn Error>> {
        let mut reader = BufReader::new(self);
        let mut vec = Vec::new();
        let n_bytes = reader.read_until(ASCII_ETX, &mut vec);
        if n_bytes == 0{
            break;
        } else {
            println!("{}", vec);
        }
        Ok("")
    }
}

pub trait MyRead {
    fn read_all_string(&mut self) -> Result<String, Box<dyn Error>>;
}