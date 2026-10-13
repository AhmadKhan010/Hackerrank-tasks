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
 
fn poisonousPlants(plants: &[i32]) -> i32 {

    let mut plants_array = plants.to_vec() ;
    let mut length = plants.len() ;
    let mut indices_to_remove: Vec<i32> = Vec::new() ;
    let mut plants_died = false ;
    let mut days = 0 ;
    
    while !plants_array.is_empty(){
        for current_index in 0..length-1 {
            if plants_array[current_index+1] > plants_array[current_index] {
                indices_to_remove.push((current_index+1) as i32) ;
                plants_died = true ;
            }
        }

        if plants_died == false {
            break ;
        }

        let remove_count = indices_to_remove.len() ;
        for remove_idx in (0..remove_count).rev() {
            plants_array.remove(indices_to_remove[remove_idx] as usize) ;
        }

        length -= remove_count;
        indices_to_remove.clear() ;  
        days += 1 ; 
        plants_died = false ;
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
