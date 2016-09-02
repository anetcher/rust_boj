fn main() {
    print_self_number();
}

fn print_self_number() {
    let n = 10000;
    let d_list: HashSet<i32> = (1..)
                            .take(n)
                            .map(|x| d(x))
                            .filter(|x| x <= &(n as i32))
                            .collect();

    println!("{}", d_list.len());
    // for num in 1..n {
    //     print_if_self_number(&(num as i32), &d_list);
    // }
}

fn print_if_self_number(num: &i32, d_list: &Vec<i32>) {
    match d_list.into_iter().find(|x| *x == num) {
        None => println!("{}", num),
        Some(_) => print!(""),
    }
}

fn d(n: i32) -> i32 {
    n + accumulate(n)
}

fn accumulate(x: i32) -> i32 {
    if x == 0 { 0 }
    else { accumulate(x / 10) + x % 10 }
}
