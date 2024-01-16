use std::io;

fn main() {
    let mut width = String::new();
    let mut length = String::new();

    println!("Please enter the width");

    io::stdin().read_line(&mut width).expect("Failed to read line");

    println!("Please enter the length");

    io::stdin().read_line(&mut length).expect("Failded to readline");

    println!("this is the width {}", width);

    println!("this is the length {}", length);



}
