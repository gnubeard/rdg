mod monster;
mod config;

use monster::Monster;
use std::error::Error;
//use std::env; //TODO args and env vars
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("/home/rlavery/.dg.toml")?; //FIXME
    let config = config::build(&contents)?;
    let my_monster = Monster::build(config.monsters)?;
    println!("{}", my_monster);
    Ok(())
}