use std::{
    io::{Error, ErrorKind, Read, Write},
    net::{IpAddr, TcpStream},
};

//                     Wont Change        Might Change            Will Change (should be same count)     Wont Change                 Number Will Be 10-15  Wont Change               Wont Change
const HEADERS: &str = "HTTP/1.1 200 OK\r\nServer: nginx/1.25.1\r\nDate: Sun, 21 Jan 2024 03:15:34 GMT\r\nContent-Type: text/plain\r\nContent-Length: 14\r\nConnection: keep-alive\r\nVary: Origin\r\n\r\n";

const IP_START: usize = HEADERS.len();
const IP_MAX_LENGTH: usize = 15;

const BUFFER_LENGTH: usize = IP_START + IP_MAX_LENGTH;

fn main() {
    match get_public_ip() {
        Ok(ip) => println!("Your public IP address is: {:?}", ip),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn get_public_ip() -> Result<IpAddr, Error> {
    // Make a TCP connection to the ipify.org API
    let mut stream = TcpStream::connect("api64.ipify.org:80")?;

    // Send an HTTP GET request
    stream.write_all(b"GET / HTTP/1.1\r\nHost: api64.ipify.org\r\n\r\n")?;

    // Read the response
    let mut response = [0; BUFFER_LENGTH];
    let ip_end = stream.read(&mut response)?;

    let response = String::from_utf8_lossy(&response[IP_START..ip_end]);

    response.parse().map_err(|_| {
        Error::new(
            ErrorKind::Other,
            "Error parsing IP address, Header formatted incorrectly",
        )
    })
}
