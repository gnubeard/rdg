# rdg
Rob's Dungeon Generator

TODO: Add docs.


## Rationale

I want a config-driven CLI tool to procedurally generate rooms for arbitrary homebrew roleplay.

Building Rooms, Monsters, NPCs, and Items are currently in scope.

Progress:

- [x] Rooms
- [x] Monsters
- [ ] Robots
- [ ] NPCs
- [ ] Environmental Hazards

## Why Rust?

Because I haven't done something in Rust before and this is about finding the right blend of cozy and challenge right now.

I'm enjoying the Rust.


## Why GPL?

Because I don't see this ever being seriously forked by anyone else anyway, and I've been using @gnubeard as a slack handle so long that it's kind of embarrassing I haven't GPLd more of my own code


send hate mail to ROB at ROBERT LAVERY dot COM

## Building

cargo test, cargo run, cargo build. Rust is great

## Contributing

Oh, thanks! This is really just a silly personal project but feel free to do whatever.
I might accept your pull request but also maybe not? This is just a fun project for me.

## Examples

lol it works on my machine

```bash
17:06:24 rlavery@novo rdg >>
cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/rdg-8dd919069ed12359)

running 2 tests
test monster::tests::monster_from_attrs ... ok
test room::tests::room_from_config ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/rdg-4f2f6b1ba1e3274c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests rdg

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

17:06:28 rlavery@novo rdg >>
cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
17:06:34 rlavery@novo rdg >>
./target/debug/rdg
Usage: ./target/debug/rdg FILE
17:06:44 rlavery@novo rdg >>
./target/debug/rdg ~/.dg.toml
Room Type: Bunk
Descriptor: Pristine
Threat 1:
  ID: 10 HP: 5
  Size: Human-ish
  Body Type: Humanoid
  Weak Point: Arm(s)
  Behavior: Stalker
  Extra Feature: Sapient
Door Count: 5
Size: Large
Set piece: There's a dead guy!
```
