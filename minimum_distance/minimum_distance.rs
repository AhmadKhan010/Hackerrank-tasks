use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;
use std::cmp::min ;

/*
 * Complete the 'minimumDistances' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

 // i32  variant
fn minimumDistances(a: &[i32]) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new() ;
    let length = a.len() ;
    let mut minimum_distance: i32 = i32::MAX; 
    
    
    for i in 0..length {
        let value = a[i] ;
        if let Some(&prev_index) = map.get(&value) {
            let distance = i as i32 - prev_index;
            minimum_distance = min(distance, minimum_distance);
        }
        
        map.insert(value, i as i32) ;
    }
    
    if minimum_distance == i32::MAX {
        -1
    }
    else {
        minimum_distance
    }
    
}

// same code with usize variant (using usize instead of casting usize to i32 everytime in the loop)
fn minimumDistances(a: &[i32]) -> i32 {
    let mut map: HashMap<i32, usize> = HashMap::new() ;
    let length = a.len() ;
    let mut minimum_distance: usize = usize::MAX; 
    
    
    for i in 0..length {
        let value = a[i] ;
        if let Some(&prev_index) = map.get(&value) {
            let distance = i - prev_index;
            minimum_distance = min(distance, minimum_distance);
        }
        
        map.insert(value, i) ;
    }
    
    if minimum_distance == usize::MAX {
        -1
    }
    else {
        minimum_distance as i32
    }
    
}


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = minimumDistances(&a);

    writeln!(&mut fptr, "{}", result).ok();
}
