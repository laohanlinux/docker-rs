use std::thread;
use std::net;
use std::io::{self, BufWriter, BufReader, BufRead, Write};

fn main() {
    let addr: net::SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let lis = net::TcpListener::bind(addr).expect("can't bind the address");
    lis.incoming().for_each(move |socket| {
        match socket {
            Ok(stream) => {
                let (rd, wd) = (stream.try_clone().unwrap(), stream.try_clone().unwrap());
                let remote = stream.peer_addr().unwrap();
                println!("{} is connected", remote.to_string());
                thread::spawn(move || {
                    let mut reader = BufReader::new(rd);
                    let mut writer = BufWriter::new(wd);
                    let mut line = String::new();
                    loop {
                        match reader.read_line(&mut line) {
                            Ok(0) => {
                                break;
                            }
                            Ok(_) => {
                                line = "You say => ".to_string() + &line;
                                writer.write_all(line.as_bytes()).unwrap();
                                writer.flush().unwrap();
                                line.clear();
                            }
                            Err(e) => {
                                eprintln!("encountered IO error: {}", e);
                                break;
                            }
                        }
                    }
                    println!("{} closed", remote.to_string());
                });
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                eprintln!("wait socket ready");
            }
            Err(e) => {
                eprintln!("encountered IO error: {}", e);
            }
        }
    });
}