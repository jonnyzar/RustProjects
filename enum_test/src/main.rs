#![allow(warnings)]
use std::fmt;


enum WebEvent {
//enum WebEvent can be only one of the variables defined below but not All at the Same Time!
    PageLoad,
    PageUnload,
    Keypress(char, bool),
    Paste(String),
    Click {x: i64, y: i64},
}

enum IpAddrKind {
    V4 (u8,u8,u8,u8),
    V6 (String),
}

enum InAddress{
    V4(String),
    V6(String),
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



fn main() {



    

    let localhost = InAddress::V4("127.0.0.0".to_string());

    //let four = IpAddrKind::V4;
    //let four1 = IpAddrKind::V4;
    //let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));



    let c = Car::Type(String::from("Jaguar"));
    let m = Car::Move{ x:15, y:19};
    c.drive();
}

