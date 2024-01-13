// Enums in rust

#[derive(Debug)]
pub enum IpAddressKind {
    // V4(u8,u8,u8,u8)
    V4(String),
    V6(String),
}

// Enums can take any kind of data inside an enum
// variant: strings, numeric types or structs.

// struct Ipv4Addr {
// // --snip--
// }
// struct Ipv6Addr {
// // --snip--
// }
// enum IpAddr {
// V4(Ipv4Addr),
// V6(Ipv6Addr),
// }

pub enum Message {
    Quit,                       //  has no data associated with it at all.
    Move { x: u32, y: u32 },    // has named fields, like a struct does.
    Speak(String),              //  includes a single String .
    ChangeColor(i32, i32, i32), // includes three i32 values
}

// There is one more similarity between enums and structs:
//just as we’re able to defi ne methods
//on structs using impl , we’re also able to defi ne methods on enums

impl  Message {
    fn call(&self){}
}

pub fn enums_exam() {
    // let home: IpAddressKind = IpAddressKind::V4(127, 0, 0, 1);
    let home: IpAddressKind = IpAddressKind::V4(String::from("127.0.0.1"));
    let loopback: IpAddressKind = IpAddressKind::V6(String::from("::01"));

    println!("===== V4: {:?} === V6: {:?} ==", &home, &loopback);

    // ==========
    
    let m = Message::Speak(String::from("HI! Hello"));
    m.call();
}

fn route(ip_kind: &IpAddressKind) -> &IpAddressKind {
    ip_kind
}
