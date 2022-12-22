

fn main() {

    println!("Lets use some structs!");

    let rect1 = Rectangle {
        a: 500,
        b: 300,
    };

    println!("rect1 is {:#?}", rect1);

    //betterway to debug structs
    dbg!(&rect1);

    let area = calc_area (&rect1);

    println!("Area equals {area}");

}

// use & to borrow struct to return its ownership to main
fn calc_area (rec: &Rectangle) -> u32 {
    return rec.a * rec.b;
} 

//wwrite derive Debug to be able to debug this structs
#[derive(Debug)]
struct Rectangle {
// rectangle dimensions in meters
    a: u32,
    b: u32,
}