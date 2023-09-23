mod monster;

use dg::Config;
use monster::Monster;
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
    let contents = fs::read_to_string(file_path)?;
    let config = Config::build(&contents)?;
    let my_monster = Monster::build(config.monsters)?;
    println!("{}", my_monster);
    Ok(())
}
