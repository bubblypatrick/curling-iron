use std::io::{Read, Write};
use std::net::TcpStream;
use anyhow::Ok;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

fn main() -> anyhow::Result<()> {
    println!("Enter a domain to GET");

    let mut input_buffer: String = String::new();
    let _ = std::io::stdin().read_line(&mut input_buffer);

    // trim input because read_line adds an \n. 
    let host = input_buffer.trim();
    if host.is_empty() {
        anyhow::bail!("no hostname supplied")
    }
    
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())?;
    let response = resolver.lookup_ip(host)?;

    for ip in response {
        let address = match ip.is_ipv6() {
            true => format!("[{}]:80", ip),
            false => format!("{}:80", ip),
        };

        match TcpStream::connect(&address) {
            std::result::Result::Ok(mut socket) => {
                let request = format!("GET / HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");
                socket.write_all(request.as_bytes())?;
                let mut body = Vec::new();
                socket.read_to_end(&mut body)?;
                println!("{}", String::from_utf8_lossy(&body));
                return Ok(());
            },
            Err(_) => continue,
        };
    }
    anyhow::bail!("Unable to connect to any addresses returned by DNS provider");
}
