use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

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

fn poisonousPlants(p: &[i32]) -> i32 {

    let mut vector = p.to_vec() ;
    let mut length = p.len() ;
    let mut flag = false ;
    let mut days = 0 ;
    
    while !vector.is_empty(){
        let mut survivors: Vec<i32> = Vec::with_capacity(vector.len()) ;
        survivors.push(vector[0]) ;

        for i in 0..length-1 {
            if vector[i+1] > vector[i] {
                flag = true ;
            }
            else {
                survivors.push(vector[i+1])
            }
        }
        
        if flag == false {
            break ;
        }

        vector = survivors ;

        length = vector.len() ;

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
