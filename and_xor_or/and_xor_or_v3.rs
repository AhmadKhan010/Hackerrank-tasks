use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'andXorOr' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

// 3) a more optimized approach of O(N) most probably in which we are not 
// re calculating the smallest indexes of numbers to the right and left, 
// instead we store the previously calculated smalles numbers in arrays 
// and if not found adjecently, we directly check in that array

fn andXorOr(numbers: &[i32]) -> i32 {
    let length = numbers.len();
    let mut max_result = 0;
    
    // arrays to store the indexes of the nearest strictly smaller elements
    let mut left_smaller = vec![-1; length];
    let mut right_smaller = vec![length as i32; length];
    
    // calculate left_smaller indices
    for current_index in 0..length {
        let mut jump_index = current_index as i32 - 1;
        while jump_index >= 0 && numbers[jump_index as usize] >= numbers[current_index] {
            jump_index = left_smaller[jump_index as usize]; 
        }
        left_smaller[current_index] = jump_index;
    }
    
    // calculate right_smaller indices
    for current_index in (0..length).rev() { 
        let mut jump_index = current_index as i32 + 1;
        while jump_index < length as i32 && numbers[jump_index as usize] >= numbers[current_index] {
            jump_index = right_smaller[jump_index as usize]; 
        }
        right_smaller[current_index] = jump_index;
    }
    
    for current_index in 0..length {
        let current_value = numbers[current_index];
        
        let left_smallest_index = left_smaller[current_index];
        if left_smallest_index >= 0 {
            let smallest = current_value;
            let second_smallest = numbers[left_smallest_index as usize];
            
            let bitwise_and = smallest & second_smallest;
            let bitwise_or = smallest | second_smallest;
            let xor_of_and_or = bitwise_and ^ bitwise_or;
            let bitwise_xor = smallest ^ second_smallest;
            let current_expression_value = xor_of_and_or & bitwise_xor;
            
            if current_expression_value > max_result {
                max_result = current_expression_value;
            }
        }
        
        let right_smallest_index = right_smaller[current_index];
        if right_smallest_index < length as i32 {
            let smallest = current_value;
            let second_smallest = numbers[right_smallest_index as usize];
            
            let bitwise_and = smallest & second_smallest;
            let bitwise_or = smallest | second_smallest;
            let xor_of_and_or = bitwise_and ^ bitwise_or;
            let bitwise_xor = smallest ^ second_smallest;
            let current_expression_value = xor_of_and_or & bitwise_xor;
            
            if current_expression_value > max_result {
                max_result = current_expression_value;
            }
        }
    }
    
    max_result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _a_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = andXorOr(&a);

    writeln!(&mut fptr, "{}", result).ok();
}



