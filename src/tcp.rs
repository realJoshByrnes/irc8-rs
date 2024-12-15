use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

use crate::handle_message;

pub fn start_server(address: &str) {
    let listener = TcpListener::bind(address).unwrap();
    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept a connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let mut cursor = 0;

    loop {
        match stream.read(&mut buffer[cursor..]) {
            Ok(0) => { // Connection closed
                println!("Client disconnected");
                break;
            }
            Ok(bytes_read) => {
                let mut start = 0;
                for i in cursor..cursor + bytes_read {
                    if buffer[i] == b'\n' {
                        let message;
                        if (i > 0) && buffer[i - 1] == b'\r' {
                            message = str::from_utf8(&buffer[start..i - 1]).unwrap();
                        } else {
                            message = str::from_utf8(&buffer[start..i]).unwrap();
                        }

                        println!("Received message: {}", message);
                        let response = handle_message(message);
                        if let Err(e) = stream.write_all(response.as_bytes()) {
                            eprintln!("Failed to send response: {}", e);
                            return;
                        }
                        start = i + 1;
                    }
                }
                if start == cursor + bytes_read {
                    cursor = 0;
                } else if start == 0 && cursor + bytes_read == buffer.len() {
                    // Our buffer is full, but doesn't contain a newline. RFC 1459 doesn't specify what to do here.

                    // According to https://modern.ircdocs.horse:
                    // - Servers SHOULD gracefully handle messages over the 512-bytes limit. They may:
                    //   - Send an error numeric back, preferably ERR_INPUTTOOLONG (417)
                    //   - Truncate on the 510th byte (and add \r\n at the end) or, preferably,
                    //       on the last UTF-8 character or grapheme that fits.
                    //   - Ignore the message or close the connection – but this may be confusing to users of buggy clients.

                    // We will just truncate the message at the 510th byte and append \r\n. This is the simplest solution.
                    let message = str::from_utf8(&buffer[..510]).unwrap();
                    println!("Received message: {}", message);
                    let response = handle_message(message);
                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        eprintln!("Failed to send response: {}", e);
                        return;
                    }

                    cursor = 0;
                } else {
                    cursor = cursor + bytes_read - start;
                    buffer.copy_within(start..cursor + bytes_read, 0);
                }
            }
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                break;
            }
        }
    }
}
