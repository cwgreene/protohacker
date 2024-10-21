use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use clap::Parser;

#[derive(Parser)]
#[clap(name="echo")]
struct Options {
    #[clap(value_parser)]
    domain : String,
    #[clap(value_parser)]
    port : String
}

fn handle_client(mut stream: TcpStream) {
    let mut buf : [u8;1] = [0;1];
    let addr = stream.peer_addr().unwrap();
    println!("Accepting Connection from {}", addr);
    loop {
        let res = stream.read(&mut buf);
        match res {
            Err(e) => println!("{}", e),
            Ok(bytes_read) =>
                if bytes_read == 0 {
                    break;
                }
        }
        _ = stream.write(&buf);
    }
    println!("Closing Connection");
}

fn main() -> std::io::Result<()> {
    let options = Options::parse();
    let listener = TcpListener::bind([options.domain, options.port].join(":"))?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                    let _handler = thread::spawn(move || {
                        handle_client(stream);
                    });
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    Ok(())
}
