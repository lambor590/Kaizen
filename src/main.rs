mod logger;
mod updater;

use crossterm::{
    cursor, execute,
    style::{Color, Stylize},
    terminal::{Clear, ClearType, SetTitle},
};
use logger::Logger;
use std::{
    io::{stdin, stdout},
    thread,
    time::Duration,
};
use updater::Updater;

fn main() {
    execute!(
        stdout(),
        SetTitle(format!("Ghost Toolkit - v{}", env!("CARGO_PKG_VERSION")))
    )
    .unwrap();

    let title: [&str; 6] = [
        "  _____ _______ ",
        " / ____|__   __|",
        "| |  __   | |   ",
        "| | |_ |  | |   ",
        "| |__| |  | |   ",
        " \\_____|  |_|   ",
    ];

    let term_width: usize = crossterm::terminal::size().map_or(80, |(w, _)| w as usize);

    let rainbow_colors: [Color; 6] = [
        Color::Red,
        Color::Magenta,
        Color::Yellow,
        Color::Green,
        Color::Cyan,
        Color::Blue,
    ];

    execute!(stdout(), cursor::Hide).unwrap();

    for color_step in (0..rainbow_colors.len() * 4).rev() {
        for &line in &title {
            let padding: usize = term_width.saturating_sub(line.len()) / 2;
            print!("{:padding$}", "", padding = padding);
            for (ch_idx, ch) in line.chars().enumerate() {
                print!(
                    "{}",
                    ch.with(rainbow_colors[(color_step + ch_idx / 2) % rainbow_colors.len()])
                );
            }
            println!();
        }
        thread::sleep(Duration::from_millis(60));
        execute!(stdout(), cursor::MoveToPreviousLine(title.len() as u16)).unwrap();
    }

    execute!(stdout(), cursor::MoveToRow(7)).unwrap();

    if let Err(e) = Updater::check_updates() {
        Logger::error(format!("Error al comprobar actualizaciones: {}", &e).as_str());
    }

    thread::sleep(Duration::from_secs(2));
    execute!(stdout(), cursor::MoveToPreviousLine(1)).unwrap();
    execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();

    stdin().read_line(&mut String::new()).unwrap();
}
