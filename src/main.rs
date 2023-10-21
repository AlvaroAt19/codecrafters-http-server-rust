// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};
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
    
    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        let _directory:Option<String> = directory.clone();
      
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                task::spawn(async move {handle_connection(_stream,_directory).await});
            }
            Err(e) => {
                println!("error: {}", e);
            }
        };

    };

}

async fn handle_connection(mut stream: TcpStream, directory: Option<String>) {
    let mut buffer: [u8; 256] = [0; 256];
    
    stream.read(&mut buffer).unwrap();
    
    let parsed_vec: Vec<&str> = std::str::from_utf8(&buffer).unwrap()
                                    .split("\r\n").collect();

    match parsed_vec[0].split(" ").collect::<Vec<&str>>()[0]{
        "GET" => handle_get(stream, directory, parsed_vec).await,
        "POST" => handle_post(stream, directory, parsed_vec).await,
        _ => {}
    }
    
}

async fn handle_get(mut stream: TcpStream, directory: Option<String>, parsed_vec: Vec<&str>){
    
    let route: &str = parsed_vec[0].split_whitespace().collect::<Vec<&str>>()[1];

    let ok_response: &str = "HTTP/1.1 200 OK\r\n\r\n";
    let error_response: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

    match route.split("/").collect::<Vec<&str>>()[1]{
        "echo" => {

            let words: String = route.replace("/echo/", "");
            let response: &str = &format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {0}\r\n\r\n{1}\r\n", words.len(), words);

            stream.write_all(response.as_bytes()).unwrap();
        },
    
        "user-agent" =>{
            let user_agent = parsed_vec[2].replace("User-Agent: ", "");
            let response: &str = &format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {0}\r\n\r\n{1}\r\n", user_agent.len(), user_agent);

            stream.write_all(response.as_bytes()).unwrap();

        },
        "files" => {
        
            let mut file_path: String = parsed_vec[0].split(" ").collect::<Vec<&str>>()[1].replace("/files/", "");
            file_path = format!("{}{}",directory.unwrap(), file_path);
            
                let file = File::open(file_path);

                match file{

                    Ok(mut file) => {
                                
                                let mut content: String = String::new();

                                file.read_to_string(&mut content).unwrap();

                                let response: &str = &format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {0}\r\n\r\n{1}\r\n",content.len(),content);
                                stream.write_all(response.as_bytes()).unwrap();
                                }

                    Err(_) => {stream.write_all(error_response.as_bytes()).unwrap();}
                };

        },
        "" => {stream.write_all(ok_response.as_bytes()).unwrap();}, 
        _ => {stream.write_all(error_response.as_bytes()).unwrap();}
    };

}

async fn handle_post(mut stream: TcpStream, directory: Option<String>, parsed_vec: Vec<&str>){
    
    let response: &str = "HTTP/1.1 201 Created\r\n\r\n";

    let route:&str =  parsed_vec[0].split(" ").collect::<Vec<&str>>()[1];
    let route = route.replace("/files/", "") ;

    let file_path = format!("{}{}",directory.unwrap(), route);


    let content = parsed_vec[6].trim();
    
    let mut file = File::create(file_path).unwrap();
    
    file.write_all(content.as_bytes()).unwrap();

    stream.write_all(response.as_bytes()).unwrap();
}