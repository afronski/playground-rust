#![warn(rust_2018_idioms)]

use tokio::net::UdpSocket;

use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7777".to_string());

    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let mut to_send = None;
    let mut buf = vec![0; 1024];

    loop {
        if let Some((size, peer)) = to_send {
            let amt = socket.send_to(&buf[..size], &peer).await?;

            if amt == 0 {
                return Ok(());
            }

            println!("Echoed {}/{} bytes to {}", amt, size, peer);
        }

        to_send = Some(socket.recv_from(&mut buf).await?);
    }
}
