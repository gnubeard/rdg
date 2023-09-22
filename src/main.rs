mod monster;
use monster::Monster;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let my_monster = Monster::build()?;
    println!("{}", my_monster);
    Ok(())
}
