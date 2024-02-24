

/** this is about enum
 *  a enum are just variance
 */


enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
        kind: IpAddrKind,
        address: String,
}



fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

}
