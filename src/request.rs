use std::fs::File;
use std::io::{Read,Write};


pub struct Request{
    connection : String,
    route: String,
    content: String,
    method: String,
    user_agent: String,
}

impl Request{
    pub fn new(connection: String, route: String, content: String,  method: String, user_agent:String) -> Self {
        Request { connection, route, content, method, user_agent }
    }

    pub fn run(&self, directory:&String) -> String{
        
        match self.method.as_str(){
            "GET" => self.get(directory),
            "POST" => self.post(directory),
            _ =>    "HTTP/1.1 405 Method Not Allowed\r\n\
                    Content-Type: text/plain\r\n\
                    Allow: GET, POST\r\n\
                    Content-Length: 20\r\n\
                    \r\n\
                    Method Not Allowed".to_string(),
        }

    }

    fn get(&self, directory:&String) -> String{

        let template: String = self.response_template();

        let response:String = match self.route.as_str(){
            
            s if s.starts_with("/echo") => {

                let words: String = self.route.replace("/echo/", "");
                template
                    .replace("-replace1-","200 OK\r\nContent-Type: text/plain")
                    .replace("-replace2-", &words.as_bytes().len().to_string()) + &words
                    
            },

            s if s.starts_with("/files") => {
            
                let file_path = format!("{}{}",directory, self.route.replace("/files/", ""));
                
                let file = File::open(file_path);

                match file{

                    Ok(mut file) => {
                                
                                let mut content: String = String::new();

                                file.read_to_string(&mut content).unwrap();

                                template
                                .replace("-replace1-","200 OK\r\nContent-Type: application/octet-stream")
                                .replace("-replace2-", &content.as_bytes().len().to_string()) + &content

                                }

                    Err(_) => {
                        template
                        .replace("-replace1-","404 Not Found")
                        .replace("-replace2-", "0")
                    },
                }

            },

                    
            "/user-agent" =>{
                // Searching for the User-Agent header in the request
                // and returning it in the response
                template
                    .replace("-replace1-","200 OK\r\nContent-Type: text/plain")
                    .replace("-replace2-", &self.user_agent.as_bytes().len().to_string()) + &self.user_agent


            },

            "/" => {
                template
                .replace("-replace1-","200 OK")
                .replace("-replace2-", "0")

            }, 
            _ => {
                template
                .replace("-replace1-","404 Not Found")
                .replace("-replace2-", "0")
            },
        };

        response
    }

    fn post(&self, directory:&String) -> String{

        let template = self.response_template();

        let route = self.route.replace("/files/", "") ;

        let file_path = format!("{}{}",directory, route);

        let mut file = File::create(file_path).unwrap();

        match file.write_all(self.content.as_bytes()){
            Ok(_) => {
                template.replace("-replace1-","201 Created").replace("replace2",   "0")
            },
            Err(_) => {
                template.replace("-replace1-","500 Internal Server Error").replace("replace2",   "0")
            }
        }

    }

    fn response_template(&self) -> String{
        format!("HTTP/1.1 -replace1-\r\nContent-Length: -replace2-\r\nConnection: {}\r\n\r\n", self.connection)
    }
}