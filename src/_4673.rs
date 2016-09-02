use std::collections::HashSet;

pub fn print_self_number(limit: i32) {
    let d_set: HashSet<i32> = (1..limit)
                                .map(|x| d(&x))
                                .filter(|x| *x < limit)
                                .collect();

    for n in 1..limit {
        if !d_set.contains(&n) {
            println!("{}", &n);
        }
    }
}

fn d(n: &i32) -> i32 {
    n + accumulate(n)
}

fn accumulate(x: &i32) -> i32 {
    if *x == 0 { 0 }
    else { accumulate(&(x / 10)) + x % 10 }
}
