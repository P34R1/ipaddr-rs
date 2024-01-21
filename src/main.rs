use crossterm::{
    terminal::{Clear, ClearType, EnterAlternateScreen, SetSize},
    ExecutableCommand,
};
use std::{
    io::{Error, ErrorKind, Read, Result, Write},
    net::{IpAddr, TcpStream},
    sync::mpsc,
    thread,
    time::Duration,
};

const TERMINAL_WIDTH: u16 = 75;
const TERMINAL_HEIGHT: u16 = 9;

fn cls(stdout: &mut std::io::Stdout) -> Result<&mut std::io::Stdout> {
    stdout.execute(Clear(ClearType::All))
}

const INTERVAL: Duration = Duration::from_millis(1000);

fn spawn_public_ip_task(tx: mpsc::Sender<IpAddr>) -> thread::JoinHandle<()> {
    // Store the initial result
    let mut current_ip = get_public_ip().expect("to get public ip");
    let _ = tx.send(current_ip); // Send a message to the channel
    thread::spawn(move || loop {
        thread::sleep(INTERVAL);

        // Call the function
        let new_ip = match get_public_ip() {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Check if the ip address has changed
        if new_ip != current_ip {
            let _ = tx.send(new_ip); // Send a message to the channel

            // Update the current ip
            current_ip = new_ip;
        }
    })
}

fn main() -> Result<()> {
    // Get stdout
    let mut stdout = std::io::stdout();

    // Create an asynchronous channel for updates
    let (tx, rx) = mpsc::channel::<IpAddr>();

    // Spawn a background task to update the channel at the specified interval
    spawn_public_ip_task(tx);

    // Setup screen
    stdout
        .execute(SetSize(TERMINAL_WIDTH, TERMINAL_HEIGHT))?
        .execute(EnterAlternateScreen)?;

    for ip_addr in rx {
        cls(&mut stdout)?;

        let ip_addr = ip_addr.to_string();

        // Calculate the x position to print the line centered
        let x = (TERMINAL_WIDTH - ip_addr.len() as u16) / 2;

        // Calculate the position for vertical centering
        let y = TERMINAL_HEIGHT / 2;

        stdout
            // Set the cursor position to the middle
            .execute(crossterm::cursor::MoveTo(x, y))?
            .execute(crossterm::style::Print(ip_addr))?;
    }

    Ok(())
}

//                     Wont Change        Might Change            Will Change (should be same count)     Wont Change                 Number Will Be 10-15  Wont Change               Wont Change
const HEADERS: &str = "HTTP/1.1 200 OK\r\nServer: nginx/1.25.1\r\nDate: Sun, 21 Jan 2024 03:15:34 GMT\r\nContent-Type: text/plain\r\nContent-Length: 14\r\nConnection: keep-alive\r\nVary: Origin\r\n\r\n";

const IP_START: usize = HEADERS.len();
const IP_MAX_LENGTH: usize = 15;

const BUFFER_LENGTH: usize = IP_START + IP_MAX_LENGTH;

fn get_public_ip() -> Result<IpAddr> {
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
