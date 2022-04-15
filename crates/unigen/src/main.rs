const DEFAULT_CLI_SIZE: u32 = 50;

fn main() {
    let parsed_size: u32 = if let Some(arg) = std::env::args().nth(1) {
        arg.trim().parse::<u32>().unwrap()
    } else {
        DEFAULT_CLI_SIZE
    };

    let _ = unigen::builder::generate_universe(parsed_size);
}
