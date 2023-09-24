# rdg
Rob's Dungeon Generator

TODO: Items, shift config validation left, arg/option dispatch to call for creation of individual elements. More TOML knobs?

## Rationale

I want a config-driven CLI tool to procedurally generate rooms for arbitrary homebrew roleplay.

Building Rooms, Monsters, NPCs, and Items are currently in scope.

Progress:

- [x] Rooms
- [x] Monsters
- [x] Robots
- [x] NPCs
- [x] Environmental Hazards
- [ ] Items

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
20:51:23 rlavery@novo rdg >>
cargo run -- ~/.dg.toml
   Compiling rdg v0.1.0 (/usr/home/rlavery/Nonsense/rdg)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/rdg /home/rlavery/.dg.toml`
SET PIECE: Something snags your legs and dangles you from the ceiling!
ROOM TYPE: Engineering
DESCRIPTOR: Pristine
DOOR COUNT: 3
SIZE: Tall
THREAT 1:
  MONSTER:
  ID: 9 HP: 2 LIMBS: 2
  SIZE: Massive
  BODY TYPE: Horror
  WEAK POINT: Pustule(s)
  BEHAVIOR: Trapper
  EXTRA FEATURE: Transforms
THREAT 2:
  MONSTER:
  ID: 5 HP: 5 LIMBS: 4
  SIZE: Human-ish
  BODY TYPE: Insectoid
  WEAK POINT: Chest(s)
  BEHAVIOR: Reproducer
  EXTRA FEATURE: Flies
THREAT 3:
  MONSTER:
  ID: 5 HP: 1 LIMBS: 5
  SIZE: Half-Pint
  BODY TYPE: Horror
  WEAK POINT: Leg(s)
  BEHAVIOR: Climber
  EXTRA FEATURE: Carapace

20:51:27 rlavery@novo rdg >>
cargo test;
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
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
```
