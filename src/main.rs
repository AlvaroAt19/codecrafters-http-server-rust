// Uncomment this block to pass the first stage
use tokio::net::{TcpListener, TcpStream};
use std::io::{Read,Write, ErrorKind};
use tokio::task;
use clap::Parser;
use std::fs::File;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    directory: Option<String>,

}

#[tokio::main]
async fn main() {

    let directory:Option<String> = Args::parse().directory;

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").await.unwrap();
    
    loop{
        match listener.accept().await {
        
            Ok((stream, _)) => {
                let _directory:Option<String> = directory.clone();
                println!("accepted new connection");
                task::spawn(async move {handle_connection(stream,_directory).await});
            }
            Err(e) => {
                println!("error: {}", e);
            }
        };
    }

}

async fn handle_connection(mut stream: TcpStream, directory: Option<String>) {
    
    let directory = directory.unwrap_or_default();
    // Reads the same stream that was passed to the function
    // and handles the request, until the stream is closed
    stream.readable().await.unwrap();
    loop{
        let mut buffer: [u8; 512] = [0; 512];

        match stream.try_read(&mut buffer) {
            Ok(0) => {
                // Client closed the connection
                break;
            }
            Ok(_) =>{
                let parsed_vec: Vec<&str> = std::str::from_utf8(&buffer).unwrap()
                                                .split("\r\n").collect();
                
                match parsed_vec[0].split(" ").collect::<Vec<&str>>()[0]{
                    "GET" => handle_get(&stream, &directory, parsed_vec).await,
                    "POST" => handle_post(&stream, &directory, parsed_vec).await,
                    _ => {}
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        }
    }
}

async fn handle_get(mut stream:&TcpStream, directory: &String, parsed_vec: Vec<&str>){
    
    let route: &str = parsed_vec[0].split_whitespace().collect::<Vec<&str>>()[1];
    println!("{:?}", parsed_vec);

    let ok_response: &str = "HTTP/1.1 200 OK\r\n\r\n";
    let error_response: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

    let mut response: String; 

    match route.split("/").collect::<Vec<&str>>()[1]{
        "echo" => {

            let words: String = route.replace("/echo/", "");
            response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {0}\r\nConnection: keep-alive\r\n\r\n{1}", words.as_bytes().len(), words);

        },
    
        "user-agent" =>{
            // Searching for the User-Agent header in the request
            // and returning it in the response
            let user_agent = parsed_vec
                    .iter()
                    .filter(|s| s.contains("User-Agent: "))
                    .collect::<Vec<&&str>>()[0]
                    .to_string()
                    .replace("User-Agent: ", "");
            
            response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {0}\r\nConnection: keep-alive\r\n\r\n{1}", user_agent.as_bytes().len(), user_agent);

        },
        "files" => {
        
            let mut file_path: String = parsed_vec[0].split(" ").collect::<Vec<&str>>()[1].replace("/files/", "");
            file_path = format!("{}{}",directory, file_path);
            
                let file = File::open(file_path);

                match file{

                    Ok(mut file) => {
                                
                                let mut content: String = String::new();

                                file.read_to_string(&mut content).unwrap();

                                response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {0}\r\nConnection: keep-alive\r\n\r\n{1}",content.as_bytes().len(),content);
                                
                                }

                    Err(_) => {response = error_response.to_string();},
                };

        },
        "" => {response = ok_response.to_string();}, 
        _ => {response = error_response.to_string();},

    };

    loop{
        match stream.try_write(response.as_bytes()) {
            Ok(_) => break,
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        }
    }

}

async fn handle_post(mut stream:&TcpStream, directory: &String, parsed_vec: Vec<&str>){
    
    let response: &str = "HTTP/1.1 201 Created\r\n\r\n";

    let route:&str =  parsed_vec[0].split(" ").collect::<Vec<&str>>()[1];
    let route = route.replace("/files/", "") ;

    let file_path = format!("{}{}",directory, route);


    let content = parsed_vec[6].trim_end_matches(char::from(0));
    
    let mut file = File::create(file_path).unwrap();
    
    file.write_all(content.as_bytes()).unwrap();

    loop{
        match stream.try_write(response.as_bytes()) {
            Ok(_) => break,
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        }
    }

}