extern crate argparse;

use tokio::net::{TcpListener, TcpStream};
use tokio::stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};
use log::{info, error};
use argparse::{ArgumentParser, Store};
use std::net::{SocketAddr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let mut address: String = String::from("127.0.0.1");
    let mut port: String = String::from("1883");

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("MQTT Broker");
        ap.refer(&mut address)
            .add_option(&["-a", "--address"], Store,
                        "IP address of the broker");
        ap.refer(&mut port)
            .add_option(&["-p", "--port"], Store,
                            "Port of the broker");
        ap.parse_args_or_exit();
    }

    let address = format!("{}:{}", address, port);

    let server: SocketAddr = address.parse().expect("Unable to parse socket address");

    let mut listener = TcpListener::bind(server.to_string()).await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // Process each socket concurrently.
            process(socket).await
        });
    }
}

async fn process(_socket: TcpStream) {
    // We're parsing each socket with the `BytesCodec` included in `tokio_util::codec`.
    let mut framed = BytesCodec::new().framed(_socket);

    // We loop while there are messages coming from the Stream `framed`.
    // The stream will return None once the client disconnects.
    while let Some(message) = framed.next().await {
        match message {
            Ok(bytes) => info!("bytes: {:?}", bytes),
            Err(err) => error!("Socket closed with error: {:?}", err),
        }
    }
    info!("Socket received FIN packet and closed connection");
}