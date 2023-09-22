mod monster;
mod config;

use monster::Monster;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let file_path = args.get(1).unwrap();
    let contents = fs::read_to_string(file_path)?;
    let config = config::build(&contents)?;
    let my_monster = Monster::build(config.monsters)?;
    println!("{}", my_monster);
    Ok(())
}
