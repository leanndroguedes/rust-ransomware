use std::process;

mod lib;

fn main() {
    lib::registry_autostart().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}
