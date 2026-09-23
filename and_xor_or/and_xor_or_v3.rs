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

fn andXorOr(a: &[i32]) -> i32 {
    let length = a.len();
    let mut result = 0;
    
    // arrays to store the indexes of the nearest strictly smaller elements
    let mut left_smaller = vec![-1; length];
    let mut right_smaller = vec![length as i32; length];
    
    // calculate left_smaller indices
    for i in 0..length {
        let mut j = i as i32 - 1;
        while j >= 0 && a[j as usize] >= a[i] {
            j = left_smaller[j as usize]; 
        }
        left_smaller[i] = j;
    }
    
    // calculate right_smaller indices
    for i in (0..length).rev() { 
        let mut j = i as i32 + 1;
        while j < length as i32 && a[j as usize] >= a[i] {
            j = right_smaller[j as usize]; 
        }
        right_smaller[i] = j;
    }
    
    for i in 0..length {
        let current_value = a[i];
        
        let left_smalles_index = left_smaller[i];
        if left_smalles_index >= 0 {
            let min1 = current_value;
            let min2 = a[left_smalles_index as usize];
            
            let first_and = min1 & min2;
            let first_or = min1 | min2;
            let first_xor = first_and ^ first_or;
            let second_xor = min1 ^ min2;
            let last_and = first_xor & second_xor;
            
            if last_and > result {
                result = last_and;
            }
        }
        
        let right_smallest_index = right_smaller[i];
        if right_smallest_index < length as i32 {
            let min1 = current_value;
            let min2 = a[right_smallest_index as usize];
            
            let first_and = min1 & min2;
            let first_or = min1 | min2;
            let first_xor = first_and ^ first_or;
            let second_xor = min1 ^ min2;
            let last_and = first_xor & second_xor;
            
            if last_and > result {
                result = last_and;
            }
        }
    }
    
    result
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



