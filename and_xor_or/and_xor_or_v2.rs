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
// 1 4 9 7 6
fn andXorOr(numbers: &[i32]) -> i32 {
    
    let length = numbers.len() ;
    let mut max_result = 0 ;

    let mut current_value = 0 ;
    let mut left_smallest_index = 0 ;
    let mut right_smallest_index = 0 ;
    let mut smallest = 0 ;
    let mut second_smallest = 0 ;
    let mut bitwise_and = 0 ;
    let mut bitwise_or = 0 ;
    let mut bitwise_xor = 0 ;
    let mut xor_of_and_or = 0 ;
    let mut current_expression_value = 0 ;
    
    for current_index in 0..length {
        current_value = numbers[current_index] ;
        
        left_smallest_index = current_index as i32 - 1 ;
        while left_smallest_index >= 0 && numbers[left_smallest_index as usize] >= current_value {
            left_smallest_index -= 1 ;
        }
        
        right_smallest_index = current_index + 1 ;
        while right_smallest_index < length && numbers[right_smallest_index] >= current_value {
            right_smallest_index += 1 ;
        }
        
        if left_smallest_index >= 0 {
            smallest = numbers[left_smallest_index as usize] ;
            second_smallest = current_value ;
            
            bitwise_and = smallest & second_smallest ;
            bitwise_or = smallest | second_smallest ;
            bitwise_xor = smallest ^ second_smallest ;
            xor_of_and_or = bitwise_and ^ bitwise_or ;
            current_expression_value = xor_of_and_or & bitwise_xor ;
            
            if current_expression_value > max_result {
                max_result = current_expression_value ;
            }
        }
        if right_smallest_index < length {
            smallest = numbers[right_smallest_index as usize] ;
            second_smallest = current_value ;
            
            bitwise_and = smallest & second_smallest ;
            bitwise_or = smallest | second_smallest ;
            bitwise_xor = smallest ^ second_smallest ;
            xor_of_and_or = bitwise_and ^ bitwise_or ;
            current_expression_value = xor_of_and_or & bitwise_xor ;
            
            if current_expression_value > max_result {
                max_result = current_expression_value ;
            }
        }
    }
    
    return max_result ;
    
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


