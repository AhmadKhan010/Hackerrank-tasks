use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::mem::swap;

/*
 * Complete the 'poisonousPlants' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY p as parameter.
 */

// 2) a slightly optimized approach in which tried to avoid .remove() function because it 
// re allocates or shift the vector after removing the element but this approach has 
// it's drawbacks as welly, like in a case where 99% plants survive, it copies all 
// the 99% plants to survivor vector every time which becomes an overhead and 1 test 
// case fails

fn poisonousPlants(plants: &[i32]) -> i32 {

    let mut plants_array = plants.to_vec() ;
    let mut survivors: Vec<i32> = Vec::with_capacity(plants.len()) ;
    let mut length = plants.len() ;
    let mut plants_died = false ;
    let mut days = 0 ;
    
    while !plants_array.is_empty(){
        survivors.clear() ;
        survivors.push(plants_array[0]) ;

        for current_index in 0..length-1 {
            if plants_array[current_index+1] > plants_array[current_index] {
                plants_died = true ;
            }
            else {
                survivors.push(plants_array[current_index+1]) ;
            }
        }
        
        if plants_died == false {
            break ;
        }

        swap(&mut plants_array, &mut survivors) ;

        length = plants_array.len() ;

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
