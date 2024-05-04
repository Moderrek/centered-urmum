use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor::MoveTo,
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};

fn main() {
    let mut stdout = stdout();
    let (width, height) = terminal::size().unwrap();
    let label = "urmom";
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(MoveTo(width / 2 - label.len() as u16 / 2, height / 2)).unwrap();
    stdout.write_all(label.as_bytes()).unwrap();
    stdout.flush().unwrap();
    thread::sleep(Duration::from_secs(5));
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.flush().unwrap();
}
