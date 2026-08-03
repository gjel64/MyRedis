use std::{env, io::{self, Write, stdin, stdout, Read}, net::TcpStream};


fn main() {
    let server_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9090".to_string());

    // CONNECTION
    let mut client = TcpStream::connect(server_addr.as_str())
        .expect(&format!("failed to connect to {}", server_addr).to_string());

    loop {
        let mut message = String::new();
        print!(">");
        let _=stdout().flush();
        stdin().read_line(&mut message).expect("Did not enter a correct string");
        if let Some('\n')=message.chars().next_back() {
            message.pop();
        }
        if let Some('\r')=message.chars().next_back() {
            message.pop();
        }

        // CONVERTING MESSAGE TO SENDABLE DATA
        let data = message.as_bytes().to_vec();

        // SENDING DATA
        client
            .write_all(&data)
            .expect("Failed to send");

        // RECEIVE DATA
        let mut buffer = [0; 1024];
        loop {
            match client.read(&mut buffer) { // blocking operation
                Ok (n) => {
                    if n == 0 {
                        println!("REDIS: Server closed the connection"); // EOF of connection
                        break;
                    }
                    
                    let s = match str::from_utf8(&buffer[0..n]) {
                        Ok(v) => v.to_string(),
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    };

                    println!("{}", s);
                    break;
                }
                Err (e) if e.kind() == io::ErrorKind::Interrupted => continue, // error that can occur without meaning anything (packet loss / connection drop)
                Err (e) => {
                    println!("ERROR: {}", e);
                    break;
                }
            }
        }
    }
}
