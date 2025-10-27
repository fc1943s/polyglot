// #![crate_name = "cli"]

mod cli;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = cli::Cli::v0;
    let app = app()();
    println!("app={}", app);
    Ok(())
}
