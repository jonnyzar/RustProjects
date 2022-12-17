# Intro

* All need to know and handy commands are going to be written down here.
* Only major interesting and different stuff than used by C and C++ going to be written here

## Updates

* Update rust
`rustup update`

## Build Essentials

* Simple compile
`rustc main.rs`

## Using cargo

Cargo is a packet manager for rust with plenty of helpful functionalities as you may see below.

```rust
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
8-bit i8 u8
16-bit i16 u16
32-bit i32 u32
64-bit i64 u64
128-bit i128 u128
arch isize usize
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

# Control Statements

## if

* always provide bool for evaluation

## loop

* infinite loop
* Loop labels must begin with a single quote: `'counter:loop{...}`

## for

* iteration through collection and other iterables like in python :)
* use range: `for number in (1..4){...}` where 4 is excluded

# Ownership concept

* memory management in RUST
* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped

```
For normal known literals we can allocate memmory and store in binary
```

```
Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.
```

* The memory must be requested from the memory allocator at runtime

`String::from` requests memory at runtime.

* We need a way of returning this memory to the allocator when we’re done with our String.

In Rust memory is free when variable gets out of scope.

```
 {
        let s = String::from("hello"); // s is valid now

        // do stuff with s
    }
    // this scope is now over, and s is no
    //longer valid
```

* Rust frees memory automatically if it points to same location at heap, so only shallow copy survives

* to make a deep clone use `clone()` but for integers and fixed types it is not necessary

```text
# Implemeted Copy Trait

All the integer types, such as u32.
The Boolean type, bool, with values true and false.
All the floating point types, such as f64.
The character type, char.
Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
```

# References

```text
A reference is like a pointer in that it’s an address we can follow to access the data stored at that address.
```

* A reference: &variable
* dereference: *variable
* the value is not droped when the refence is out of scope
* to modify a reference it has to be mutable

```rust
#create mutable variable

let mut a = 5;

# define function accepting mutable variables

fn func_xyz(x: &mut i32) {
    ...}

#pass reference to function

func_xyz(&a);

```

## Slices

* string slice is a reference to a part of a string `let str = &s[starting_index..ending_index];`
* type for string slice: `&str`
