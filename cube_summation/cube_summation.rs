use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap ;

/*
 * Complete the 'cubeSum' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. STRING_ARRAY operations
 */

fn cubeSum(cube_size: i32, operations: &[String]) -> Vec<i64> {
    
    let mut blocks: HashMap<(i32, i32, i32), i64> = HashMap::new() ;
    let mut results: Vec<i64> = Vec::new() ;

    let mut parts: Vec<&str> = Vec::new() ;
    let mut start_x: i32 = 0 ;
    let mut start_y: i32 = 0 ;
    let mut start_z: i32 = 0 ;
    let mut end_x: i32 = 0 ;
    let mut end_y: i32 = 0 ;
    let mut end_z: i32 = 0 ;
    let mut sum: i64 = 0 ;
    
    for current_operation in operations {
        parts = current_operation.split_whitespace().collect() ;
        
        if parts.is_empty(){
            continue;
        }
        
        if parts[0] == "UPDATE"{
            blocks.insert(
                (parts[1].parse().unwrap(), parts[2].parse().unwrap(), parts[3].parse().unwrap()),
                parts[4].parse().unwrap(),
            ) ;
        }
        else if parts[0] == "QUERY" {
            start_x = parts[1].parse().unwrap() ;
            start_y = parts[2].parse().unwrap() ;
            start_z = parts[3].parse().unwrap() ;
            end_x = parts[4].parse().unwrap() ;
            end_y = parts[5].parse().unwrap() ;
            end_z = parts[6].parse().unwrap() ;
            
            sum = 0 ;
            for (&(coord_x, coord_y, coord_z), value) in &blocks {
                if (coord_x >= start_x) && (coord_x <= end_x) && (coord_y >= start_y) && (coord_y <= end_y) && (coord_z >= start_z) && (coord_z <= end_z){
                    sum = sum + *value;
                }
            }
            results.push(sum) ;
        }
    }
    
    return results;
    
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let T = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..T {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let matSize = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let mut ops: Vec<String> = Vec::with_capacity(m as usize);

        for _ in 0..m {
            let ops_item = stdin_iterator.next().unwrap().unwrap();
            ops.push(ops_item);
        }

        let res = cubeSum(matSize, &ops);

        for i in 0..res.len() {
            write!(&mut fptr, "{}", res[i]).ok();

            if i != res.len() - 1 {
                writeln!(&mut fptr).ok();
            }
        }

        writeln!(&mut fptr).ok();
    }
}
