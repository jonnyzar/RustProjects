use std::fmt; //use fmt module

struct TupleStruct (i32,f64);

impl fmt::Display for TupleStruct {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}, {}", self.0, self.1)
    }
}

fn main() {

    let pu = TupleStruct(23, 4.5); // initiating struct

    println!("({})", pu);
}
