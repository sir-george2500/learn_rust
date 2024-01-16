use std::{io, f32};

fn main() {
    let mut width = String::new();
    let mut length = String::new();

    const CONVERSION_FACTOR :f32 = 10.764;

    println!("Please enter the width");

    io::stdin().read_line(&mut width).expect("Failed to read line");

    println!("Please enter the length");

    io::stdin().read_line(&mut length).expect("Failded to readline");


    //shadowing
    let width :f32 = width.trim().parse().expect("Fail to parse width");
    let length:f32 = length.trim().parse().expect("Fail to parse height");

    let area = width * length;

    let area_in_square_ft = area / CONVERSION_FACTOR;

    println!("the area is {} sq ft", area_in_square_ft);




}
