use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'largestRectangle' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts INTEGER_ARRAY h as parameter.
 */

// 2) a more optimized approach in which the test cases are passing in the specific time limit and no time constraint occur on any test case.
fn largestRectangle(h: &[i32]) -> i64 {
    let length = h.len() ;
    let mut result: i64 = 0 ;
    
    for i in 0..length {
        let current_value = h[i] ;
        let mut left_largest_index = i as i32;
        
        while left_largest_index >= 0 && h[left_largest_index as usize] >=current_value {
            left_largest_index -= 1 ;
        }
        
        let mut right_largest_index = i;
        while right_largest_index < length && h[right_largest_index] >= current_value {
            right_largest_index += 1 ;
        }
        
        let count_of_buildings = right_largest_index as i32 - left_largest_index - 1 ;
        
        let area: i64 = count_of_buildings as i64 * current_value as i64;
        
        if area > result {
            result = area;
        }
    }
    
    return result;
    
}


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let h: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = largestRectangle(&h);

    writeln!(&mut fptr, "{}", result).ok();
}
