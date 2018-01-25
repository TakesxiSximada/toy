extern crate clap;

use std::net::{TcpListener, TcpStream};
use std::thread;

use std::io::Read;
use std::io::Write;

use clap::{App, Arg};


fn handle_client(mut stream: TcpStream) {
    let mut buf;
    loop {
        buf = [0; 1];
        let _ = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    break;
                }
                m
            },
        };

        match stream.write(&buf) {
            Err(_) => break,
            Ok(_) => continue,
        }
    }
}


fn main() {
    let matches = App::new("simple")
        .arg(Arg::with_name("host")
             .help("Host name")
             .short("a")
             .long("host")
             .takes_value(true)
        )
        .arg(Arg::with_name("port")
             .help("Port Number")
             .short("p")
             .long("port")
             .takes_value(true)
        ).get_matches();

    let host = matches.value_of("host").unwrap();
    let port = matches.value_of("port").unwrap();
    let addr = format!("{}:{}", host, port);
    println!("Listen: {}",  addr);

    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        match stream {
            Err(e) => { println!("failed: {}", e) },
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                });
            },
        }
    }
}
