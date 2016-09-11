use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let n:usize = buffer.trim().parse::<usize>().unwrap();

    let result = (1..).take(n)
                    .filter(|&x| is_hs(x, calc_diff(x)))
                    .count();

    println!("{:?}", result);
}

fn is_hs(x: i32, d: i32) -> bool {
    if x < 10 { true }
    else { calc_diff(x) == d && is_hs(x / 10, d) }
}

fn calc_diff(x: i32) -> i32 {
    let a = x % 10;
    let b = x % 100 / 10;
    a - b
}
