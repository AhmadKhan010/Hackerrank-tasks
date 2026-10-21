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

fn largestRectangle(heights: &[i32]) -> i64 {
    let length = heights.len() ;
    let mut max_area: i64 = 0 ;
    
    let mut current_height: i32;
    let mut left_bound_index: i32;
    let mut right_bound_index: usize;
    let mut count_of_buildings: i32;
    let mut area: i64;
    
    for current_index in 0..length {
        current_height = heights[current_index] ;
        left_bound_index = current_index as i32 - 1;
        
        while left_bound_index >= 0 && heights[left_bound_index as usize] >= current_height {
            left_bound_index -= 1 ;
        }
        
        right_bound_index = current_index +1 ;
        while right_bound_index < length && heights[right_bound_index] >= current_height {
            right_bound_index += 1 ;
        }
        
        count_of_buildings = right_bound_index as i32 - left_bound_index - 1 ;
        
        area = count_of_buildings as i64 * current_height as i64;
        
        max_area = max_area.max(area);
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
