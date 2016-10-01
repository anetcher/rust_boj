use std::io::stdin;
use std::io::prelude::BufRead;

fn main() {
    let stdin = stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse::<usize>().unwrap();
    let test_results: Vec<String> = stdin
                                    .lock()
                                    .lines()
                                    .take(n)
                                    .map(|line| line.unwrap())
                                    .collect();
    
    for result in test_results {
        let mut cs: i32 = 0;
        let mut count: i32 = 0;
        for c in result.chars() {
            if c == 'O' {
                cs += 1;
            } else {
                cs = 0;
            }
            count += cs;
        }
        println!("{}", count);
    }
}
