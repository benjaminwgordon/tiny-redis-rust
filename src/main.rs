use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

mod RESP;
mod command;
mod resp;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6380").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

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

                // Parse the command from RESP into
                // let cmd = command::command::COMMAND::from_bytes(&buf[0..n]);
                let resp_array = resp::resp::RESP::array_from_bytes(&buf[..n]);

                println!("{:#?}", resp_array);

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
