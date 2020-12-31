# Advent of Rust 🎄

Rust solutions to the [Advent of Code](https://adventofcode.com) (AoC) programming puzzles. Someday
I might even find the time to finish them all.

## Goals

In roughly descending order:

* Have fun solving meaningless problems 🎉
* Practice writing readable, hopefully idiomatic Rust code. If I were optimizing for time, I'd write
  [hacky Python](https://github.com/kesyog/adventofcode2020) instead. I'm still fairly new to Rust
  though.
* Find excuses to experiment with various crates and techniques.
* Don't look up direct hints or solutions until I've solved the problem on my own.
* Write enough tests to provide confidence, but don't go overboard with input validation. Test the
  given input as a bare minimum. Similarly, don't bother spending time on "proper" error handling.
  Just panic.
* Generally try to optimize for runtime speed, keeping in mind rule #1. At the very least, avoid
  totally naive brute-force if I can come up with a better method.

## Running the solutions

First, install [Rust + Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Each solution has an associated binary in the repo. You can let Cargo tell you what's available
via:

```sh
cargo run --release
```

To run the solution to Day 1 of 2020:

```sh
cargo run --release --bin 2020-day1
```

