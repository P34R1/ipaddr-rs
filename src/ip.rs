use std::{
    io::{Error, ErrorKind, Read, Result, Write},
    net::{IpAddr, TcpStream},
    sync::mpsc,
    thread,
    time::Duration,
};
const BUFFER_LENGTH: usize = 250;
const CRLF_OFFSET: usize = 4;

macro_rules! continue_if_err {
    ($a:expr) => {
        match $a {
            Ok(v) => v,
            _ => continue,
        }
    };
}

const FIND_IP_START: fn(&[u8]) -> usize = |response| {
    std::str::from_utf8(response)
        .expect("valid utf8")
        .find("\r\n\r\n")
        .expect("crlfcrlf to be in string")
        + CRLF_OFFSET
};

fn get_public_ip() -> Result<IpAddr> {
    // Make a TCP connection to the ipify.org API
    let mut stream = TcpStream::connect("api.ipify.org:80")?;

    // Send an HTTP GET request
    stream.write_all(b"GET / HTTP/1.1\r\nHost: api.ipify.org\r\n\r\n")?;

    // Read the response
    let mut response = [0; BUFFER_LENGTH];
    let ip_end = stream.read(&mut response)?;
    let ip_start = FIND_IP_START(&response);

    let response = String::from_utf8_lossy(&response[ip_start..ip_end]);

    response.parse().map_err(|_| {
        Error::new(
            ErrorKind::Other,
            "Error parsing IP address, Header formatted incorrectly",
        )
    })
}

const INTERVAL: Duration = Duration::from_millis(1000);

pub fn spawn_get_ip_task(tx: mpsc::Sender<IpAddr>) -> thread::JoinHandle<()> {
    // Store the initial result
    let mut current_ip = get_public_ip().expect("to get public ip");
    let _ = tx.send(current_ip); // Send a message to the channel
    thread::spawn(move || loop {
        thread::sleep(INTERVAL);

        // Call the function
        let new_ip = continue_if_err!(get_public_ip());

        // Check if the ip address has changed
        if new_ip != current_ip {
            let _ = tx.send(new_ip); // Send a message to the channel

            // Update the current ip
            current_ip = new_ip;
        }
    })
}
