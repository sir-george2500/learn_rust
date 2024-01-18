fn multiple_of_3_and_5(number:i32) -> i32 {

    let mut sum = 0;
    let mut i = 1;

    while i <= number {

        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
        i += 1;
    }
    sum

}


fn main() {
   let result =  multiple_of_3_and_5(49);

   println!("This is the result {}", result);
}
