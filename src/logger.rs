use crossterm::style::Stylize;

pub struct Logger;

impl Logger {
    pub fn info(message: &str) {
        println!(
            "{} {}",
            " INFO ".bold().black().on_blue(),
            message
        );
    }

    pub fn warn(message: &str) {
        println!(
            "{} {}",
            " WARN ".bold().black().on_yellow(),
            message
        );
    }

    pub fn error(message: &str) {
        println!(
            "{} {}",
            " ERROR ".bold().black().on_red(),
            message
        );
    }
}
