use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'largestRectangle' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts INTEGER_ARRAY h as parameter.
 */

 // 1) naive approach in which it runs on O(n^2) and most of the test cases are failing due to time limit constraint.
fn largestRectangle(h: &[i32]) -> i64 {
    let length = h.len();
    let mut result: i64 = 0;
    
    for i in 0..length {
        let mut local_min = h[i];
        
        let mut local_result: i64 = local_min as i64 * 1;
        if local_result > result {
            result = local_result;
        }
        
        let mut counter = 1;
        for j in (i + 1)..length {
            let value = h[j];
            if value < local_min {
                local_min = value;
            }
            counter += 1;
            
            local_result = local_min as i64 * counter as i64;
            if local_result > result {
                result = local_result;
            }
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
