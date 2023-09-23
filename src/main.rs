use dg::run;
use std::error::Error;
use std::fs;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let dollar_zero = &args[0]; // deserves to panic
    let file_path = args.get(1).unwrap_or_else(|| {
        eprintln!("Usage: {} FILE", dollar_zero);
        process::exit(1);
    });
    let config_file_data = fs::read_to_string(file_path)?;
    run(&config_file_data)?;
    Ok(())
}
