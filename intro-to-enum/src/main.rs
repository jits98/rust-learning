fn main() {
    println!("Hello, world!");
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call (&self) {

    }
}

let m = Message::Write(String::from("hello"));
m.call();

// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }

// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32);

// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// let home = IpAddr::V4(127,0,0,1);

// let loopback = IpAddr::V6(String::from("::1"));

// let home = IpAddr::V4(String::from("127.0.0.1"));

// let loopback = IpAddr::V6(String::from("::1"));

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };

// let four = IpAddrKind::V4;
// let six = IpAddrKind::V6;

// fn route(ip_Kind: IpAddrKind) {}

// route(IpAddrKind::V4);
// route(IpAddrKind::V6);