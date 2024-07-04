mod logger;

use logger::Logger;

fn check_update() -> Result<(), Box<dyn ::std::error::Error>> {
    // TODO: Implementar
    Ok(())
}

fn main() {
    Logger::info("Primera versión pública de Ghost Toolkit hecho en Rust");

    check_update().unwrap();

    std::io::stdin().read_line(&mut String::new()).unwrap();
}
