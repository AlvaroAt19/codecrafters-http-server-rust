use std::fs::File;
use std::io::{Read,Write};


pub struct Request{
    connection : String,
    route: String,
    content: String,
    method: String,
    user_agent: String,
    encoding: String,
}

impl Request{
    pub fn new(connection: String, route: String, content: String,  method: String, user_agent:String, encoding:String) -> Self {
        Request { connection, route, content, method, user_agent, encoding }
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
        
        let mut replace1:String;
        let mut body:String = String::new();

        match self.route.as_str(){
            
            s if s.starts_with("/echo") => {

                body = self.route.replace("/echo/", "");
                
                replace1 = "200 OK\r\nContent-Type: text/plain".to_string(); 

            }

            s if s.starts_with("/files") => {
            
                let file_path = format!("{}{}",directory, self.route.replace("/files/", ""));
                
                let file = File::open(file_path);

                match file{

                    Ok(mut file) => {

                        file.read_to_string(&mut body).unwrap();

                        replace1 = "200 OK\r\nContent-Type: application/octet-stream".to_string() ;

                        }

                    Err(_) => {
                        replace1 = "404 Not Found".to_string();

                    },
                }

            },

                    
            "/user-agent" =>{
                // Searching for the User-Agent header in the request
                // and returning it in the response
                replace1 = "200 OK\r\nContent-Type: text/plain".to_string();
                body = self.user_agent.clone();
                    
            },

            "/" => {
                replace1 = "200 OK".to_string();

            }, 
            _ => {
                replace1 = "404 Not Found".to_string();
            },
        };


        match self.find_encoding(){
            "gzip" => {
                replace1 = format!("{}\r\nContent-Encoding: gzip", replace1);
            },
            "deflate" => {
                replace1 = format!("{}\r\nContent-Encoding: deflate", replace1);
            },
            _ => {}
        }

        template
            .replace("-replace1-", &replace1)
            .replace("-replace2-", &body.len().to_string())
            + body.as_str()

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

    fn find_encoding(&self) -> &str{
        
        let client_encodings : Vec<&str> = self.encoding
        .split(",")
        .map(| s| s.trim()) 
        .collect();
        
        let server_encodings: Vec<&str> = vec!["gzip", "deflate"];

        for encoding in client_encodings.iter(){
            
            if server_encodings.contains(encoding){
                return encoding;
            }

        }

        return "invalid";
    }
    
}