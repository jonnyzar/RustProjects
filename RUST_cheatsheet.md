# Intro

All need to know and handy commands are going to be written down here.

# Updates
* Update rust
`rustup update`

# Build Essentials

* Simple compile
`rustc main.rs`

## Using cargo

Cargo is a packet manager for rust with plenty of helpful functionalities as you may see below.

```
# create new project 
cargo new

# check if project builds without errors
cargo new

# just build into src/debug


# build and run project
cargo run

# build optimized release into src/release
cargo build --release

# update dependencies bypassig Cargo.lock
cargo update

# get documentation for all referenced dependencies
cargo doc --open

```