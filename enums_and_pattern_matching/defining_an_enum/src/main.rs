/** this is about enum
 *  a enum are just variance
 */


enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}



fn main() {
 let home = IpAddrKind::V4(127, 0, 0, 1);
}
