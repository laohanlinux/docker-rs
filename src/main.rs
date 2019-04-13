use tokio::prelude::*;
use tokio::net::TcpListener;
use tokio::codec::{Decoder, LinesCodec};

fn main() {
    let addr = "127.0.0.1:3000".parse().expect("couldn't parse address");
    let listener = TcpListener::bind(&addr).expect("couldn't bind address");
    let future = listener
        .incoming()
        .for_each(|socket| {
            let lines_codec = LinesCodec::new();
            let (sink, stream) = lines_codec.framed(socket).split();
            sink
                .send(String::from("Welcome to the echo server"))
                .and_then(|sink| {
                    let stream = stream
                        .map(|line| format!("You said: {}", line))
                        ;
                    sink.send_all(stream)
                        .map(|_| println!("Connection closed"))
                })
        })
        .map_err(|e| eprintln!("An error occurred: {:?}", e))
        ;
    tokio::run(future)
}
