use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'poisonousPlants' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY p as parameter.
 */

 // 1) it passes all the test cases and is the optimized version 
 // because here the remove function uses memmove at the backend and is 
 // optimized in the rust compiler which move the vector elements in the block 
 // format like SIMD and not one by one, due to which it is more optimized than 
 // the survivors approach
 
fn poisonousPlants(p: &[i32]) -> i32 {

    let mut vector = p.to_vec() ;
    let mut length = p.len() ;
    let mut index_of_remove_plants: Vec<i32> = Vec::new() ;
    let mut flag = false ;
    let mut days = 0 ;
    
    while !vector.is_empty(){
        for i in 0..length-1 {
            if vector[i+1] > vector[i] {
                index_of_remove_plants.push((i+1) as i32) ;
                flag = true ;
            }
        }

        if flag == false {
            break ;
        }

        let n = index_of_remove_plants.len() ;
        for i in (0..n).rev() {
            vector.remove(index_of_remove_plants[i] as usize) ;
        }

        length -= n;
        index_of_remove_plants.clear() ;  
        days += 1 ; 
        flag = false ;
    }
    
    return days;

}


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let p: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = poisonousPlants(&p);

    writeln!(&mut fptr, "{}", result).ok();
}
