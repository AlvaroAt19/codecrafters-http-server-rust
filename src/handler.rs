use clap::builder::Str;
use tokio::net:: TcpStream;
use std::io::ErrorKind;
use crate::request::Request;


pub struct Handler{
    stream: TcpStream,
    directory: Option<String>,
}

impl Handler{
    pub fn new(stream: TcpStream, directory: Option<String>) -> Self {
        Handler { stream, directory}
    }


    fn parse(buffer:[u8; 1024]) -> Vec<String> {
        let request_vec: Vec<&str> = std::str::from_utf8(&buffer).unwrap()
                                                    .split("\r\n").collect();

        let method: String = request_vec[0].split(" ").collect::<Vec<&str>>()[0].to_string();

        let route: String = request_vec[0].split(" ").collect::<Vec<&str>>()[1].to_string();

        let connection: String = request_vec
                    .iter()
                    .filter(|s| s.contains("Connection: "))
                    .collect::<Vec<&&str>>()[0]
                    .to_string()
                    .replace("Connection: ", "");

        let content: String = request_vec.last().unwrap().trim_end_matches(char::from(0)).to_string();

        let user_agent = request_vec
                    .iter()
                    .filter(|s| s.contains("User-Agent: "))
                    .collect::<Vec<&&str>>()[0]
                    .to_string()
                    .replace("User-Agent: ", "");



        
        vec![connection, route, content, method, user_agent]
    }   
    
    pub async fn handle(&self) {
        let directory: String = self.directory.clone().unwrap_or_default();
        // Reads the same stream that was passed to the function
        // and handles the request, until the stream is closed
        loop{
            self.stream.readable().await.unwrap();

            let mut buffer: [u8; 1024] = [0; 1024];

            match self.stream.try_read(&mut buffer) {
                Ok(0) => {
                    // Client closed the connection
                    break;
                }
                
                Ok(_) =>{
                    
                    let parsed_vec: Vec<String> = Self::parse(buffer);

                    let response: String = Request::new(parsed_vec[0].clone(), parsed_vec[1].clone(), parsed_vec[2].clone(), parsed_vec[3].clone(), parsed_vec[4].clone())
                                            .run(&directory);
                    
                    loop{
                        match self.stream.try_write(response.as_bytes()) {
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
}