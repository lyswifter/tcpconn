use tokio::{
    net::TcpStream,
    net::TcpListener,
    sync::{mpsc, oneshot, RwLock},
    task,
    time,
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

    //127.0.0.1:8080
    //172.28.234.45:4132
    //8.219.132.75:4132
    //144.126.219.193:4132

    println!("-------start connection--------");

    let stream = TcpStream::connect("172.28.234.45:4132").await;

    println!("---------------");

    match stream {
        Ok(mut streamm) => {
            let ret = streamm.write_all(b"hello world!").await;
            println!("-------+++--------");
            match ret {
                Ok(_rett) => {
                    println!("write hello ok");
                }
                Err(error) => {
                    println!("write info encounter error {}", error)
                }
            }
        }
        Err(error) => {
            println!("connection encounter error {}", error)
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
