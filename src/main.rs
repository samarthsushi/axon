use std::env;
use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::time::Duration;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!("usage: client <port> <message> [timeout_in_seconds]");
        return Ok(());
    }

    let port: u16 = match args[1].parse() {
        Ok(p) if (1..=65535).contains(&p) => p,
        _ => {
            eprintln!("error: port number must be an integer between 1 and 65535.");
            return Ok(());
        }
    };
    let message = &args[2];

    let timeout_secs: u64 = if args.len() == 4 {
        match args[3].parse() {
            Ok(t) if t > 0 => t,
            _ => {
                eprintln!("error: timeout must be a positive integer representing seconds.");
                return Ok(());
            }
        }
    } else {
        5 // default timeout in seconds
    };

    let address = format!("127.0.0.1:{}", port);
    let timeout = Duration::new(timeout_secs, 0);

    match TcpStream::connect_timeout(&address.parse().unwrap(), timeout) {
        Ok(mut stream) => {
            println!("connected to: {}", address);
            stream.set_read_timeout(Some(timeout))?;
            stream.set_write_timeout(Some(timeout))?;

            stream.write_all(message.as_bytes())?;
            println!("sent: {}", message);

            let mut buffer = [0; 512];
            match stream.read(&mut buffer) {
                Ok(bytes) if bytes > 0 => {
                    println!("recvd: {}", String::from_utf8_lossy(&buffer[..bytes]));
                }
                Ok(_) => {
                    println!("no response recvd");
                }
                Err(e) => eprintln!("error: {}", e),
            }
        }
        Err(e) => eprintln!("error: {}", e),
    }

    Ok(())
}
