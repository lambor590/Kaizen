pub struct Console;

use crossterm::{
    cursor, execute,
    style::{Color, Stylize},
    terminal::{Clear, ClearType, SetTitle},
};
use std::{io::stdout, thread, time::Duration};

use crate::{
    commands::{Activator, Cleaner},
    logger::Logger,
};

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

    pub fn animate_logo(fast: bool) {
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

        execute!(stdout(), &cursor::MoveTo(0, 0), &cursor::Hide).unwrap();

        let speed: usize = if fast { 2 } else { 4 };

        for color_step in (0..&rainbow_colors.len() * speed + title.len()).rev() {
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
        let mut countdown: i32 = 5;

        execute!(stdout(), &cursor::Hide, &cursor::MoveToNextLine(1)).unwrap();

        while countdown >= 0 {
            Self::sleep_secs(1);
            Logger::info(&format!("Volviendo al menú en {} segundos...", &countdown));
            execute!(stdout(), &cursor::MoveToPreviousLine(1)).unwrap();
            countdown -= 1;
        }
    }

    pub fn menu() {
        let options: Vec<&str> = vec![
            "Limpiar archivos temporales",
            "Activar Windows de forma permanente",
        ];

        let action: Result<&str, inquire::InquireError> =
            inquire::Select::new("¿Qué quieres hacer?", options)
                .with_help_message("Selecciona una opción usando las flechas y pulsando Enter")
                .prompt();

        match action {
            Ok("Limpiar archivos temporales") => Cleaner::run(),
            Ok("Activar Windows de forma permanente") => Activator::run(),
            _ => Logger::error("Opción no reconocida."),
        }

        Self::press_any_key();
        Self::animate_logo(true);
    }
}
