fn main() {
    if let Err(e) = catr::run(catr::get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
