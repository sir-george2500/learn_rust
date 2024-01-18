// Function to calculate the sum of multiples of 3 or 5 up to a given number
fn multiple_of_3_and_5(number: i32) -> i32 {
    // Initialize the sum to 0
    let mut sum = 0;
    // Initialize a counter starting from 1, as 0 is divisible by every number
    let mut i = 1;

    // Iterate while 'i' is less than the specified 'number'
    while i < number {
        // Check if 'i' is divisible by 3 or 5
        if i % 3 == 0 || i % 5 == 0 {
            // If divisible, add 'i' to the sum
            sum += i;
        }
        // Increment 'i' to move to the next number
        i += 1;
    }

    // Return the calculated sum
    sum
}

// Main function
fn main() {
    // Call the function with the argument 49 and store the result
    let result = multiple_of_3_and_5(49);

    // Print the result
    println!("This is the result {}", result);
}

