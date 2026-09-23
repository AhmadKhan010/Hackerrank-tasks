use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'andXorOr' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */




// 2) another approach in which checking only relevant valid pairs by checking the smallest number in the neighbourhood of the target element.
// this approach lowers the number of total pairs it checks, but still 6 test cases are failing due to time constraints which are too large and complex and take more time.
fn andXorOr(a: &[i32]) -> i32 {
    
    let length = a.len() ;
    let mut result = 0 ;
    
    for i in 0..length {
        let current_value = a[i] ;
        
        let mut left_smalles_index = i as i32 -1;
        while left_smalles_index >= 0 && a[left_smalles_index as usize] >= current_value {
            left_smalles_index -= 1 ;
        }
        
        let mut right_smallest_index = i + 1 ;
        while right_smallest_index < length && a[right_smallest_index] >= current_value{
            right_smallest_index += 1 ;
        }
        
        if left_smalles_index >= 0 {
            let min1 = current_value ;
            let min2 = a[left_smalles_index as usize] ;
            
            let first_and = min1 & min2 ;
            let first_or = min1 | min2 ;
            let first_xor = first_and ^ first_or ;
            let second_xor = min1 ^ min2;
            let last_and = first_xor & second_xor ;
            
            if last_and > result {
                result = last_and;
            }
        }
        if right_smallest_index < length{
            let min1 = current_value ;
            let min2 = a[right_smallest_index as usize] ;
            
            let first_and = min1 & min2 ;
            let first_or = min1 | min2 ;
            let first_xor = first_and ^ first_or ;
            let second_xor = min1 ^ min2;
            let last_and = first_xor & second_xor ;
            
            if last_and > result {
                result = last_and;
            }
        }
    }
    
    return result ;
    
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


