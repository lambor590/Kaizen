pub struct Console;

use crossterm::{
    cursor, execute,
    style::{Color, Stylize},
    terminal::{Clear, ClearType, SetTitle},
};
use std::{
    io::{stdin, stdout, Read},
    thread,
    time::Duration,
};

use crate::{commands::Cleaner, logger::Logger};

impl Console {
    pub fn set_title(message: &str) {
        execute!(stdout(), SetTitle(&message)).unwrap();
    }

    pub fn clear() {
        execute!(
            stdout(),
            cursor::RestorePosition,
            Clear(ClearType::FromCursorDown)
        )
        .unwrap();
    }

    pub fn animate_logo() {
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

        execute!(stdout(), cursor::MoveTo(0, 0), cursor::Hide).unwrap();

        for color_step in (0..&rainbow_colors.len() * 4 + title.len()).rev() {
            for &line in &title {
                let padding: usize = (term_width - line.len()).saturating_div(2);
                print!("{:width$}", "", width = &padding);
                for (ch_idx, ch) in line.chars().enumerate() {
                    print!(
                        "{}",
                        ch.with(rainbow_colors[(color_step + &ch_idx / 2) % rainbow_colors.len()])
                    );
                }
                println!();
            }
            Self::sleep_ms(60);
            execute!(stdout(), &cursor::MoveToPreviousLine(title.len() as u16)).unwrap();
        }

        Self::clear();
    }

    pub fn sleep_ms(duration: u64) {
        thread::sleep(Duration::from_millis(duration));
    }

    pub fn sleep_secs(duration: u64) {
        thread::sleep(Duration::from_secs(duration));
    }

    pub fn press_any_key() {
        Logger::info("Presiona una tecla para continuar...");
        stdin().read_exact(&mut [0u8]).unwrap();

        Self::clear();
    }

    pub fn menu() {
        loop {
            let options: Vec<&str> = vec![
                "Limpiar archivos temporales",
                "Activar Windows de forma permanente",
            ];
            let action: Result<&str, inquire::InquireError> =
                inquire::Select::new("¿Qué quieres hacer?", options)
                    .with_help_message("Selecciona una opción usando las flechas y pulsando Enter")
                    .prompt();

            Self::clear();

            match &action {
                Ok("Limpiar archivos temporales") => Cleaner::run().unwrap(),
                Ok("Activar Windows de forma permanente") => {
                    Logger::info("Activando Windows de forma permanente...")
                }
                _ => Logger::warn("Opción no reconocida."),
            }

            Self::press_any_key();
            Self::animate_logo();
        }
    }
}
