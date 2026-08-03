use std::{env, io::{self, Read, Write}, net::{TcpListener, TcpStream}, thread};
use myredis::database::Database;

fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream
        .peer_addr()
        .map_or_else(|_| "unknown".to_string(), |addr| addr.to_string()); // default : unknown but if we have adress => take address

    println!("REDIS: Handling connection from {}", peer_addr);

    let mut buffer = [0; 1024]; // fixed sized byte array

    let mut db = Database::new();

    // MAIN LOOP ----------
    loop {
        match stream.read(&mut buffer) { // blocking operation
            Ok (n) => {
                if n == 0 {
                    println!("REDIS: Client {} closed the connection", peer_addr); // EOF of connection
                    break;
                }
                
                let s = match str::from_utf8(&buffer[0..n]) {
                    Ok(v) => v.to_string(),
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };

                println!("REDIS: Received from client {} : {}", peer_addr, s);
                
                // HANDLE COMMAND --------
                let data = match db.exec_command(s) {
                    Ok(s) => s.as_bytes().to_vec(),
                    Err(s) => s.as_bytes().to_vec()
                };

                //println!("{}", str::from_utf8(&data).unwrap()); --> to debug

                // SENDING DATA
                stream.write_all(&data).expect("Failed to send");
            }
            Err (e) if e.kind() == io::ErrorKind::Interrupted => continue, // error that can occur without meaning anything (packet loss / connection drop)
            Err (e) => {
                match e.kind() {
                    io::ErrorKind::ConnectionReset => {
                        println!("REDIS: Client {} reset connection", peer_addr);
                    }
                    _ => {
                        eprintln!("REDIS: Read error from client {} : {}", peer_addr, e)
                    }
                }
                break;
            }
        }
    }

    println!("REDIS: Connection finished for {}", peer_addr);

    // dont need to clean stream because it goes out of scope (function takes the ownership)
}


fn main() {
    println!("REDIS: Server starting");
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9090".to_string());

    let listener = TcpListener::bind(&addr).expect("REDIS: Failed to bind to address"); // failed if invalid port / used port

    println!("REDIS: Server listening on {}", addr);
    for stream_result in listener.incoming() // blocking operation
    {
        match stream_result {
            Ok(stream) => {
                thread::spawn(move || { // move : give ownership of the stream variable
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("REDIS: Failed to accept connection : {}", e)
            }
        }
    }
}
