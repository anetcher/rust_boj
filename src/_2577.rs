use std::io::stdin;
use std::io::prelude::BufRead;

fn main() {
    let stdin = stdin();
    let number = stdin
                    .lock()
                    .lines()
                    .take(3)
                    .map(|x| x.unwrap().trim().parse::<u32>().unwrap())
                    .fold(1, |x, y| x * y);

    let mut counts: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for n in number.to_string().chars() {
        counts[n.to_digit(10).unwrap() as usize] += 1;
    }

    for c in counts {
        println!("{}", c);
    }
}
