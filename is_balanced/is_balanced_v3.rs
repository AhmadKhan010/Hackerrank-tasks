use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'isBalanced' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */


 // 3) A more cleaner version in which used match instead of if else, 
fn isBalanced(s: &str) -> String {
    // using traditional vector as a stack
    let mut stack: Vec<char> = Vec::new() ;


    for current_char in s.chars() {
        match current_char {
            '(' | '[' | '{' => stack.push(current_char),
            
            ')' => if stack.pop() != Some('(') {return "NO".to_string()},
            ']' => if stack.pop() != Some('[') {return "NO".to_string()},
            '}' => if stack.pop() != Some('{') {return "NO".to_string()},
            
            _ => {}
        }
    }
    
    if stack.is_empty() {
        return "YES".to_string();
    }
    else {
        return "NO".to_string() ;
    }
}


fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let s = stdin_iterator.next().unwrap().unwrap();

        let result = isBalanced(&s);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
