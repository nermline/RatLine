use std::io::{self, Write, Read};
use std::net::{TcpStream};

fn main() {
    let server_addr = "192.168.31.10:8080";
    
    let mut stream = TcpStream::connect(server_addr).expect("Cannot connect to the server!");

    println!("Connected to the {}", server_addr);
    
    loop {
        print!("Enter message: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Cannot read the message!");
    
        stream.write_all(input.as_bytes()).expect("Cannot send message!");
    
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).expect("Cannot read unswer from server!");
    
        let response = String::from_utf8_lossy(&buffer[..n]);
        println!("Unswer from server: {}", response);
    }
}