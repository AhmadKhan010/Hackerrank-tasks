use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

/*
 * Complete the 'sherlockAndAnagrams' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

fn sherlockAndAnagrams(s: &str) -> i32 {
    let mut string_counts: HashMap<String, i32> = HashMap::new() ;
    let length = s.len() ;
    
    for i in 0..length {
        for j in i+1..=length {
            let substring = &s[i..j] ;
            
            let mut chars: Vec<char> = substring.chars().collect() ;
            chars.sort() ;
            let sorted_stirng: String = chars.into_iter().collect() ;
            
            let count = string_counts.entry(sorted_stirng).or_insert(0) ;
            *count+=1;
            
            
        }
    }
    
    
    let mut total_anagram_pairs = 0;
    for value in string_counts.values(){
        total_anagram_pairs += value * (value-1) / 2;
        
    }
    
    total_anagram_pairs
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let s = stdin_iterator.next().unwrap().unwrap();

        let result = sherlockAndAnagrams(&s);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
