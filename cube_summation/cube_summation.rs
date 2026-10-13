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
    let num_operations = operations.len() ;
    let mut results: Vec<i64> = Vec::new() ;
    
    for op_index in 0..num_operations {
        let current_operation = &operations[op_index] ;
        
        let parts: Vec<&str> = current_operation.split_whitespace().collect() ;
        
        if parts.is_empty(){
            continue;
        }
        
        if parts[0] == "UPDATE"{
            let coord_x: i32 = parts[1].parse().unwrap() ;
            let coord_y: i32 = parts[2].parse().unwrap() ;
            let coord_z: i32 = parts[3].parse().unwrap() ;
            let value: i64 = parts[4].parse().unwrap() ;
            
            blocks.insert((coord_x, coord_y, coord_z), value) ;
        }
        else if parts[0] == "QUERY" {
            let start_x: i32 = parts[1].parse().unwrap() ;
            let start_y: i32 = parts[2].parse().unwrap() ;
            let start_z: i32 = parts[3].parse().unwrap() ;
            let end_x: i32 = parts[4].parse().unwrap() ;
            let end_y: i32 = parts[5].parse().unwrap() ;
            let end_z: i32 = parts[6].parse().unwrap() ;
            
            let mut sum: i64 = 0 ;
            for (block, value) in &blocks {
                let &(coord_x, coord_y, coord_z) = block ;
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
