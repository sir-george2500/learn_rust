use core::f32;
use std::io;

fn main() {
    let mut width = String::new();
    let mut length = String::new();

    println!("Please enter the width");

    io::stdin().read_line(&mut width).expect("Failed to read line");

    println!("Please enter the length");

    io::stdin().read_line(&mut length).expect("Failded to readline");


    //shadowing
    let width :f32 = width.trim().parse().expect("Fail to parse width");
    let length:f32 = length.trim().parse().expect("Fail to parse height");

    println!("this is the width {}", width);

    println!("this is the length {}", length);



}
