// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_connection(_stream).expect("Failed to handle_connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let mut buffer: [u8; 128] = [0; 128];
    
    stream.read(&mut buffer)?;
    
    let parsed_vec: Vec<&str> = std::str::from_utf8(&buffer).unwrap()
                                    .split("\r\n").collect();

    let route: &str = parsed_vec[0].split_whitespace().collect::<Vec<&str>>()[1];

    let ok_response: &str = "HTTP/1.1 200 OK\r\n\r\n";
    let error_response: &str = "HTTP/1.1 404 Not Found\r\n\r\n";
    
    match route{
        "/" => stream.write(ok_response.as_bytes())?, 
        _ => stream.write(error_response.as_bytes())?
    };
    
    Ok(())
}
