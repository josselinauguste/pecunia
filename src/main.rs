use crate::io::cli;

mod application;
mod domain;
mod infrastructure;
mod io;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match cli::handle_command(&args) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
