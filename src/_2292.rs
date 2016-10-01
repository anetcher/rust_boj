use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let n = stdin.lock().lines().next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    println!("{:?}", min_path(n - 1, 1));
}

fn min_path(n: i32, i: i32) -> i32 {
    if n <= 0 { i }
    else { min_path(n - i * 6, i + 1) }
}
