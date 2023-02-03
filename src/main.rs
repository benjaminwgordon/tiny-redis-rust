use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

mod DB;
mod command;
mod resp;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init DB
    let counter = Arc::new(Mutex::new(DB::DB::new()));

    let listener = TcpListener::bind("127.0.0.1:6380").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        let counter = Arc::clone(&counter);

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Parse the command from RESP into in-memory representation
                let resp_array = resp::resp::RESP::array_from_bytes(&buf[..n])
                    .expect("TODO: Error handling for RESP array creation");

                // parse in-memory RESP to Redis Command
                let command = command::command::COMMAND::from_resp_array(&resp_array)
                    .expect("TODO: Error handling for Command parse from RESP Array");

                // wait for the lock on the DB to become available, then take it
                let mut db = counter.lock().await;
                let out = command.execute(&mut db);
                println!("{}", out);

                // Write the data back
                if let Err(e) = socket.write_all(&out.as_bytes()).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
