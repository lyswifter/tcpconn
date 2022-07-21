use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot, RwLock},
    task,
    time::timeout,
};
use tokio::io::AsyncWriteExt;
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};
use std::net::{IpAddr, Ipv4Addr};

use log::{info, error};

#[tokio::main]
async fn main() {
    // let peer_ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(172,28,234,45)), 4132);

    let stream = TcpStream::connect("172.28.234.45:4132").await;

    match stream {
        Ok(mut streamm) => {
            let ret = streamm.write_all(b"hello world!").await;
            match ret {
                Ok(_rett) => {
                    info!("write hello ok");
                }
                Err(error) => {
                    error!("write info encounter error {}", error)
                }
            }
        }
        Err(error) => {
            error!("connection encounter error {}", error)
        }
    }

    println!("Hello, end!");

    // match TcpStream::connect(peer_ip) {
    //     Ok(_stream) => {
    //         info!("Connect ok '{}'", peer_ip);
    //     },
    //     Err(error) => {
    //         error!("Unable to reach '{}': '{:?}'", peer_ip, error);
    //     }
    // }
}
