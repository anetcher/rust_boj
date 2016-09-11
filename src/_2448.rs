use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let n:i32 = buffer.trim().parse::<i32>().unwrap();

    // let result = (1..).take(n)
    //                 .filter(|&x| is_hs(x, calc_diff(x)))
    //                 .count();
    print_basic_star(n);
    // println!("{:?}", result);
}

// fn print_star(x: i32, y: i32) {
//     print_star(x/2, y);
//     print_star(x, y);
// }

fn print_basic_star(x: i32) {
    let mut indent = String::new();
    for i in 0..x { indent.push(' '); }
    println!("{}  *", indent);
    println!("{} * *", indent);
    println!("{}*****", indent);
}
