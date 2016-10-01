use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let x = stdin.lock().lines().next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let max: i32 = 10 * 1000 * 1000;
    let lines: Vec<i32> = (1..).take((max as f32).sqrt() as usize)
                                .map(|n| (n * (n + 1)) / 2)
                                .collect();

    let line: i32 = match lines.binary_search(&x) {
        Ok(n) => n as i32,
        Err(n) => n as i32,
    };
    let offset = lines[line as usize] - x;
    let a: i32 = match line % 2 {
        0 => 1 + offset,
        1 => line + 1 - offset,
        _ => 0,
    };
    let b: i32 = line + 2 - a;
    println!("{}/{}", a, b);
}
