fn main() {
    if let Err(e) = transr::run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
