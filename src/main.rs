use std::{env, error::Error};

use notify_rust::Notification;
use tokio;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let listener = TcpListener::bind(&addr).await?;

    println!("ntfn {} running on {}", env!("CARGO_PKG_VERSION"), addr);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = socket.read(&mut buf).await.expect("Failed to read data from socket");

                if n == 0 { return; }

                let s = String::from_utf8(buf[0..n].to_vec()).unwrap();

                match s.trim().as_ref() {
                    "foo" => {
                        Notification::new().summary("foo").body("blah blah").timeout(0).show().unwrap();
                    },
                    "bar" => {
                        println!("bar called");
                    },
                    _ => {
                        println!("unknown command");
                    },
                }
            }
        });
    }
}