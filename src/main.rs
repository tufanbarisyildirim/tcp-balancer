mod upstream;

use clap::{App, Arg};
use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use crate::upstream::Upstream;


fn main() -> io::Result<()> {

    let matches = App::new("Dummy TCP MITM")
        .arg(Arg::with_name("destinations")
            .short('d')
            .long("destinations")
            .value_name("DESTINATIONS")
            .help("List of destination IP:port pairs, separated by commas")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("listen_port")
            .short('p')
            .long("listen-port")
            .value_name("PORT")
            .help("Port to listen on")
            .takes_value(true)
            .required(true))
        .get_matches();

    // Listen on localhost:8000
    let destinations: Vec<&str> = matches.value_of("destinations").unwrap().split(",").collect();
    let  upstreams: Vec<Upstream> = destinations.iter().map(|d| Upstream::new(d)).collect();

    let listen_port = matches.value_of("listen_port").unwrap();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", listen_port))?;
    let mut dest_index = 0;


    // Accept incoming connections
    for stream in listener.incoming() {
        let stream = stream?;
        println!("Incoming connection from {}", stream.peer_addr()?);

        let mut dest = &upstreams[dest_index.clone() % destinations.len()];
        dest_index += 1;
        let mut iterations = 0;
        while !dest.is_live {
            dest_index += 1;
            dest = &upstreams[dest_index.clone() % destinations.len()];
            iterations += 1;
            if iterations >= upstreams.len() {
                break;
            }
        }
        if !dest.is_live {
            panic!("No live upstreams are available.")
        }

        println!("{}:{}", dest.ip,dest.port);
        dest_index += 1;

        // Connect to the destination IP:port
        let dest_stream = TcpStream::connect(format!("{}:{}",dest.ip,dest.port))?;
        let source_peer_addr = stream.peer_addr().unwrap();
        let dest_peer_addr = dest_stream.peer_addr().unwrap();


        // Forward data in both directions
        let source = stream.try_clone()?;
        let destination = dest_stream.try_clone()?;
        thread::spawn(move || {
            let _ = forward_data(source, destination, source_peer_addr, dest_peer_addr);
        });

        let destination_reverse = stream.try_clone()?;
        let source_reverse = dest_stream.try_clone()?;
        thread::spawn(move || {
            let _ = forward_data(source_reverse, destination_reverse, dest_peer_addr, source_peer_addr);
        });
    }
    Ok(())
}

fn forward_data(mut reader: TcpStream, mut writer: TcpStream, from: std::net::SocketAddr, to: std::net::SocketAddr) -> io::Result<()> {
    let mut buf = [0; 1024];
    loop {
        let len = reader.read(&mut buf)?;
        if len == 0 {
            println!("Connection closed from {}", from);
            break;
        }
        // Print data to the console
        println!("Data from {} to {}: {:?}", from, to, &buf[..len]);
        // Write data to the destination
        writer.write_all(&buf[..len])?;
    }
    Ok(())
}