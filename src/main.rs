// Uncomment this block to pass the first stage
mod handler;
mod request;

use crate::handler::Handler;
use tokio::net::TcpListener;
use tokio::task;
use clap::Parser;

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
                task::spawn(async move {Handler::new(stream,_directory).handle().await});
            }
            Err(e) => {
                println!("error: {}", e);
            }
        };
    }

}