use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::VecDeque;

/*
 * Complete the 'minimumMoves' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. STRING_ARRAY grid
 *  2. INTEGER startX
 *  3. INTEGER startY
 *  4. INTEGER goalX
 *  5. INTEGER goalY
 */

fn minimumMoves(grid: &[String], startX: i32, startY: i32, goalX: i32, goalY: i32) -> i32 {
    let n = grid.len() ;
    
    let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new() ;
    
    let mut visited = vec![vec![false; n]; n] ;
    
    queue.push_back((startX, startY, 0));
    visited[startX as usize][startY as usize] = true ;
    
    let possible_directions = [(-1,0), (1,0), (0,1), (0,-1)] ;
    
    while !queue.is_empty(){  
        
        let (curr_X, curr_Y, moves) = queue.pop_front().unwrap() ;
        
        if curr_X == goalX && curr_Y == goalY {
            return moves;
        }
        
        for (x,y) in possible_directions {
            let mut next_X = curr_X + x ;
            let mut next_Y = curr_Y + y ;
            
            while next_X >=0 && next_X < (n as i32) && next_Y >= 0 && next_Y < (n as i32) && grid[next_X as usize].as_bytes()[next_Y as usize] != b'X' {
                
                if !visited[next_X as usize][next_Y as usize] {
                    visited[next_X as usize][next_Y as usize] = true ;
                    
                    queue.push_back((next_X, next_Y, moves+1));
                }
                
                next_X += x ;
                next_Y += y ;
                
            }
        }
        
    }
    
    -1
    
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut grid: Vec<String> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let grid_item = stdin_iterator.next().unwrap().unwrap();
        grid.push(grid_item);
    }

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let startX = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let startY = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let goalX = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let goalY = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let result = minimumMoves(&grid, startX, startY, goalX, goalY);

    writeln!(&mut fptr, "{}", result).ok();
}
