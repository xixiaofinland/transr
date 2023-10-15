fn main() {
    if let Err(e) = transr::get_args().and_then(transr::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
