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
fn largestRectangle(heights: &[i32]) -> i64 {
    let length = heights.len();
    let mut max_area: i64 = 0;
    
    let mut min_height: i32;
    let mut current_area: i64;
    let mut width: i32;
    let mut height: i32;
    
    for start_index in 0..length {
        min_height = heights[start_index];
        
        current_area = min_height as i64;
        if current_area > max_area {
            max_area = current_area;
        }
        
        width = 1;
        for end_index in (start_index + 1)..length {
            height = heights[end_index];
            if height < min_height {
                min_height = height;
            }
            width += 1;
            
            current_area = min_height as i64 * width as i64;
            if current_area > max_area {
                max_area = current_area;
            }
        }
    }
    return max_area;
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
