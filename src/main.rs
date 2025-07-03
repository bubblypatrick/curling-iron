use std::io::{Read, Write};
use std::net::TcpStream;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

fn main() {
    let mut input_buffer: String = String::new();
    let _ = std::io::stdin().read_line(&mut input_buffer);

    // trim input because read_line adds an \n. 
    let host = input_buffer.trim();
    
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let response = resolver.lookup_ip(host).expect("Unable to find a DNS record for provided input");

    let address = response.iter().next().expect("No addresses resturned!");
    if address.is_ipv4() {
        println!("Returned ipv4");
    } else {
        println!("Returned ipv6");
    }

    match TcpStream::connect(format!("{address}:80")) {
        Ok(mut stream) => {
            let request = format!("GET / HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");

            stream.write_all(request.as_bytes()).expect("Connected alright, but unable to write to byte stream");

            let mut response = String::new();
            stream.read_to_string(&mut response).expect("Sent a GET request, but couldn't read response stream");

            println!("Response: {response}");
        },
        Err(_) => println!("Unable to bind to port 80 of provided dns."),
    }
}
