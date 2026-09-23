fn simple_array_sum(ar: &[i32]) -> i32 {
    let length = ar.len(); 
    let mut sum = 0;
    for i in 0..length {
        sum+=ar[i]
    }
    
    sum 
}



fn main() {
    // 1. Create a dummy array to test with
    let test_array = vec![1, 2, 3, 4, 10, 11];

    // 2. Call your function
    let result = simple_array_sum(&test_array);

    // 3. Print the result to your terminal
    println!("The sum is: {}", result);
}
