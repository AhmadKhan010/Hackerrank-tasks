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
fn andXorOr(a: &[i32]) -> i32 {
    
    let length = a.len() ;

    let mut result = 0 ;
    
    for i in 0..length {
        let mut min1 = a[i];
        let mut min2 = i32::MAX ;
        
        for j in (i+1)..length {
            let j_current_value = a[j] ;
            
            if j_current_value < min1 {
                min2 = min1 ;
                min1=j_current_value;
            }
            else if j_current_value < min2 && j_current_value >min1 {
                min2 = j_current_value ;   
            } 
            
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



