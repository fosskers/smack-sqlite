use gumdrop::{Options, ParsingStyle};

/// Hit an SQLite database.
#[derive(Debug, Options)]
struct Args {
    /// The identity of this executable at runtime.
    #[options(required)]
    id: usize,
}

fn main() {
    let args = Args::parse_args_or_exit(ParsingStyle::AllOptions);

    println!("ID: {}", args.id);
}
