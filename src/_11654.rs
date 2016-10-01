use std::io::stdin;
use std::io::Read;

fn main() {
    let stdin = stdin();
    println!("{:?}", stdin.lock().bytes().next().unwrap().unwrap());
}
