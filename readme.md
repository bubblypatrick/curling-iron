# Curling Iron

A dead simple CLI that can handle GET requests within rust.
Demonstrates handling and transitioning a TCP stream. Its only
dependency is on a DNS resolver to transition the host to an IPv4 or IPv6 address,
and anyhow for easy error bubbling.

# Usage

## You have rust installed already
```
cargo run

www.google.com
```

## Run it in a container
```
docker build -t curling-iron .
docker run -it curling-iron
www.google.com
```