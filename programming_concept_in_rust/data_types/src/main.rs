use std::io;

fn main() {
    /* A Simple Currency Converter that convert
     * from USD to LRD
     *
     */

    const CONVERSION_FACTOR: f32 = 187.0;
    let mut user_currency = String::new();

    println!("Please enter the LRD value");
    io::stdin().read_line(&mut user_currency).expect("Failed to read line");

    let user_currency: f32 = user_currency.trim().parse().expect("Failed to parse");

    let result = user_currency / CONVERSION_FACTOR;

    println!("Your USD is  {}", result);
}

