fn main() {


    for i in 1..100 {
        if i % 15 == 0 {
            println!("fuzz_buzz");
        }else if  i % 3 == 0 {
            println!("buzz");
        }else if i % 5 == 0 {
            println!("fuzz");
        }else{
            println!("this is the number {}", i);
        }
    }
}
