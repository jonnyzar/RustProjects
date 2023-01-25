#![allow(warnings)]

fn main() {
    //let four = IpAddrKind::V4;
    //let four1 = IpAddrKind::V4;
    //let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let c = Car::Type(String::from("Jaguar"));
    let m = Car::Move{ x:15, y:19};
    c.drive();
}

enum IpAddrKind {
    V4 (u8,u8,u8,u8),
    V6 (String),
}

enum Car{
    Start,
    Stop,
    Move {x: i32, y: i32},
    Type (String),
    Color (i32, i32, i32),
}

impl Car {
    fn drive(&self) {
        //define method
    
    }
}
