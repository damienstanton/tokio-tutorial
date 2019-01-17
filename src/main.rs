use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

fn main() {
    let addr = "127.0.0.1:9999".parse().unwrap();
    let client = TcpStream::connect(&addr)
        .and_then(|stream| {
            println!("within stream");
            io::write_all(stream, "OK!\n").then(|result| {
                println!("wrote to stream; success={:?}", result.is_ok());
                Ok(())
            })
        })
        .map_err(|err| {
            println!("an error occurred: {:?}", err);
        });

    println!("executing tokio stream");
    tokio::run(client);
}
