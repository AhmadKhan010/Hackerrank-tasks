
fn mini_max_sum(arr: &[i32]) {
    let length = arr.len();
    let mut min = i64::MAX ;
    let mut max = i64::MIN ;
    let mut sum: i64 = 0 ;
    
    for i in 0..length {
        let value = arr[i] as i64;
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
        sum += value;    
    }
    let min_sum = sum - max;
    let max_sum = sum - min;
    
    println!("{} {}", min_sum, max_sum);
    
}

fn main() {

    let arr = [1, 2, 3, 4, 5, 6, 7, 8,9];

    mini_max_sum(&arr);
}
