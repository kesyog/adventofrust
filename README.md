# Advent of Rust 🎄

Rust solutions to the [Advent of Code](https://adventofcode.com) (AoC) programming puzzles written
for practice.

## Goals

In roughly descending order:

* Have fun 🎉
* Write readable, idiomatic code. If I were optimizing for time, I'd write [hacky Python](https://github.com/kesyog/adventofcode2020)
  instead. Still pretty new to Rust so "idiomatic" is based on what feels right.
* Don't look up hints or other answers until I've solved the problem on my own.
* Write enough tests to provide confidence, but don't go overboard with input validation. Test the
  given input as a bare minimum. Similarly, don't bother spending time on "proper" error handling.
  Just panic.
* Avoid naive brute-force if there's a clearly better method

## Running the solutions

### Prerequisites

[Rust + Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Instructions

Each solution has an associated binary in the repo, so you can let Cargo tell you what's available
via:

```sh
cargo run
```

To run the solution to Day 1 of 2020:

```sh
cargo run --release --bin 2020-day1
```

