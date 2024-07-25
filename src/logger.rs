use yansi::Paint;

pub struct Logger;

impl Logger {
    pub fn info(message: &str) {
        println!(
            "{} {}",
            " INFO ".bold().on_black().bright_blue().invert(),
            message
        );
    }

    pub fn warn(message: &str) {
        println!(
            "{} {}",
            " WARN ".bold().on_black().bright_yellow().invert(),
            message
        );
    }

    pub fn error(message: &str) {
        println!(
            "{} {}",
            " ERROR ".bold().on_black().bright_red().invert(),
            message
        );
    }
}
