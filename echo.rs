use std::net::{TcpListener, TcpStream};
use std::io::{stdout, Read, Write};
use std::thread;

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
        stream.write(&buf);
    }
    println!("Closing Connection");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                    let handler = thread::spawn(move || {
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
