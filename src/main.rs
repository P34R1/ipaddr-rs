use crossterm::{
    cursor, execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize},
};
use std::{io, sync::mpsc};

mod formatting;
mod ip;

const TERMINAL_WIDTH: u16 = 75;
const TERMINAL_HEIGHT: u16 = 36;

const SIGINT_HANDLER: fn() = || {
    // Get stdout
    let mut stdout = io::stdout();

    // Reset the screen, ignoring any errors
    let _ = execute! {
        stdout,
        ResetColor,
        LeaveAlternateScreen,
        cursor::Show
    };

    // Terminate the program
    std::process::exit(0);
};

fn main() -> io::Result<()> {
    // Get stdout
    let mut stdout = io::stdout();

    // Create an asynchronous channel for updates
    let (tx, rx) = mpsc::channel();

    // Spawn a background task to update the channel
    ip::spawn_get_ip_task(tx);

    // Setup screen
    execute! {
        stdout,
        SetSize(TERMINAL_WIDTH, TERMINAL_HEIGHT),
        EnterAlternateScreen,
        cursor::Hide,
        SetAttribute(Attribute::Bold),
    }?;

    // Set up the SIGINT handler
    ctrlc::set_handler(SIGINT_HANDLER).expect("Error setting SIGINT handler");

    // Recieve values from the channel
    for ip_addr in rx {
        let ip_addr = ip_addr.to_string();

        // Calculate the position to print the line centered
        let x = (TERMINAL_WIDTH - ip_addr.len() as u16) / 2;

        // Set the cursor position to the middle and print the ip address
        execute! {
            stdout,
            // cls
            Clear(ClearType::All),
            // Reset Cursor
            cursor::MoveTo(0, 0),
            // Print the BORDER
            Print(formatting::BORDER.concat()),
            // Cursor to middle
            cursor::MoveTo(x, formatting::LINE_TO_PRINT),
            SetForegroundColor(Color::DarkBlue),
            Print(ip_addr),
        }?;
    }

    Ok(())
}
