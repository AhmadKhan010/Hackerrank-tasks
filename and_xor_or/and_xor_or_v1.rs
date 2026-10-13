use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'andXorOr' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */


 // 1) a naive approach in which many test cases are passing but some are failing due to time limit constraint,
 //  this O(N^2) time complexity solution
fn andXorOr(numbers: &[i32]) -> i32 {
    
    let length = numbers.len() ;

    let mut max_result = 0 ;
    
    for start_index in 0..length {
        let mut smallest = numbers[start_index];
        let mut second_smallest = i32::MAX ;
        
        for end_index in (start_index+1)..length {
            let current_value = numbers[end_index] ;
            
            if current_value < smallest {
                second_smallest = smallest ;
                smallest=current_value;
            }
            else if current_value < second_smallest && current_value > smallest {
                second_smallest = current_value ;   
            } 
            
            let bitwise_and = smallest & second_smallest ;
            let bitwise_or = smallest | second_smallest ;
            let xor_of_and_or = bitwise_and ^ bitwise_or ;
            let bitwise_xor = smallest ^ second_smallest;
            let current_expression_value = xor_of_and_or & bitwise_xor ;
            
            if current_expression_value > max_result {
                max_result = current_expression_value;
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



