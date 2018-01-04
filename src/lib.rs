extern crate ansi_escapes;
extern crate serde_yaml;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;

const EOL: [u8; 2] = [0x0d, 0x0a];


pub fn serve() {

    let file = File::open("game_data.yaml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut game_data: BTreeMap<String, String> = serde_yaml::from_reader(buf_reader).unwrap();

    let listener = TcpListener::bind("127.0.0.1:6480").unwrap();
    println!("Listening on port 6480");

    for incoming in listener.incoming() {
        let stream = incoming.unwrap();
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);

        writer.write_fmt(format_args!("{}", ansi_escapes::CursorHide));
        writer.write_fmt(format_args!("{}", ansi_escapes::EraseScreen));
        writer.write_fmt(format_args!("{}", ansi_escapes::CursorTo::TopLeft));
        writer.write_fmt(format_args!("{}", ansi_escapes::CursorShow));
        writer.write(&game_data.get("intro").unwrap().as_bytes());
        writer.write(&EOL).unwrap();
        writer.flush().unwrap();

        loop {
            writer.write(&EOL).unwrap();
            writer.write("> ".as_bytes()).unwrap();
            writer.flush().unwrap();

            let mut data = String::new();
            reader.read_line(&mut data).unwrap();

            let command = data.trim_right().to_lowercase();
            match command.as_ref() {
                "end" => {
                    stream.shutdown(Shutdown::Both).unwrap();
                    break;
                }
                "help" => {
                    writer
                        .write(&game_data.get("help").unwrap().as_bytes())
                        .unwrap();
                    writer.write(&EOL).unwrap();
                    writer.flush().unwrap();

                }
                _ => {
                    writer.write("Unknown command: ".as_bytes()).unwrap();
                    writer.write(command.as_bytes()).unwrap();
                    writer.write(&EOL).unwrap();
                    writer.flush().unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
