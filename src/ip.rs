use std::{
    io::{Error, ErrorKind, Read, Result, Write},
    net::{IpAddr, TcpStream},
    sync::mpsc,
    thread,
    time::Duration,
};

//                     Wont Change        Might Change            Will Change (should be same count)     Wont Change                 Number Will Be 10-15  Wont Change               Wont Change
const HEADERS: &str = "HTTP/1.1 200 OK\r\nServer: nginx/1.25.1\r\nDate: Sun, 21 Jan 2024 03:15:34 GMT\r\nContent-Type: text/plain\r\nContent-Length: 14\r\nConnection: keep-alive\r\nVary: Origin\r\n\r\n";
const IP_START: usize = HEADERS.len();

//                           225.225.225.225
const IP_MAX_LENGTH: usize = 15;
const BUFFER_LENGTH: usize = IP_START + IP_MAX_LENGTH;

macro_rules! continue_if_err {
    ($a:expr) => {
        match $a {
            Ok(v) => v,
            _ => continue,
        }
    };
}

fn get_public_ip() -> Result<IpAddr> {
    // Make a TCP connection to the ipify.org API
    let mut stream = TcpStream::connect("api.ipify.org:80")?;

    // Send an HTTP GET request
    stream.write_all(b"GET / HTTP/1.1\r\nHost: api.ipify.org\r\n\r\n")?;

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
