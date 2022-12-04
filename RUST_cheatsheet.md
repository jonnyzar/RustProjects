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

# Variables

## Scalar Types

Basic single type variables.

* integer

signed vvalues
`-(2^(n - 1)) to 2^(n - 1) - 1`

unsinged

`0 to 2^n - 1`

```
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize
```

* floating-point
* Booleans
* characters

Rust’s char type is four bytes in size and represents a Unicode Scalar Value

## Compound Type

Variable grouping multiple values of multiple types together.

### Tuple

Fixed length once declared

### Array

* Fixed length, all variables of same type
* An array is a single chunk of memory of a known, fixed size that can be allocated on the stack
* out-of-bounds index access in rust is impossbile... it wont compile and panic

```
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

### Vector

Like array but may grow and shrink in size

