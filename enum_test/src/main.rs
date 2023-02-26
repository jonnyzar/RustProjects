#![allow(warnings)]
use std::fmt;


enum WebEventControl {
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


type web = WebEventControl;


fn main() {

    let press = web::Keypress('a', true);
    let load_pg = web::PageLoad;
    let unload_pg = web::PageUnload;

}

fn define_intf () {

    let localhost = InAddress::V4("127.0.0.0".to_string());

    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));

}