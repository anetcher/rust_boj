use std::io::stdin;

pub fn print_star() {
    let mut n = String::new();
    let mut N: i32 = 0;
    if stdin().read_line(&mut n).unwrap() > 0 {
        match n.trim().parse::<i32>() {
            Ok(x) => {
                println!("{:?}", x);
                N = x;
            }
            Err(x) => println!("{:?}", x),
        }
    }

    for i in 0..N {
        println!("*");
    }
}
