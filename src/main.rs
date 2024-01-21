use crossterm::{
    terminal::{Clear, ClearType, EnterAlternateScreen, SetSize},
    ExecutableCommand,
};
use std::{io::Result, net::IpAddr, sync::mpsc};

mod ip;

const TERMINAL_WIDTH: u16 = 75;
const TERMINAL_HEIGHT: u16 = 9;

fn cls(stdout: &mut std::io::Stdout) -> Result<&mut std::io::Stdout> {
    stdout.execute(Clear(ClearType::All))
}

fn main() -> Result<()> {
    // Get stdout
    let mut stdout = std::io::stdout();

    // Create an asynchronous channel for updates
    let (tx, rx) = mpsc::channel::<IpAddr>();

    // Spawn a background task to update the channel
    ip::spawn_get_ip_task(tx);

    // Setup screen
    stdout
        .execute(SetSize(TERMINAL_WIDTH, TERMINAL_HEIGHT))?
        .execute(EnterAlternateScreen)?;

    for ip_addr in rx {
        cls(&mut stdout)?;

        let ip_addr = ip_addr.to_string();

        // Calculate the position to print the line centered
        let x = (TERMINAL_WIDTH - ip_addr.len() as u16) / 2;
        let y = TERMINAL_HEIGHT / 2;

        // Set the cursor position to the middle and print the ip address
        stdout
            .execute(crossterm::cursor::MoveTo(x, y))?
            .execute(crossterm::style::Print(ip_addr))?;
    }

    Ok(())
}
