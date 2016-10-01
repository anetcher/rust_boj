use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let n: i32 = buffer.trim().parse::<i32>().unwrap();
    let w: usize = n as usize * 2 - 1;

    let mut spaces = String::new();
    for _ in 0..w { spaces.push(' '); }

    let mut stars = f(0, n - 1, n);
    stars.push((n, 0));
    stars.sort();

    let mut by = 0;
    let mut line = spaces.clone();
    for star in stars {
        let y = star.0;
        let x = star.1;

        if y != by {
            println!("{}", line.split_at(w).0);
            line = spaces.clone();
        }

        line.insert(x as usize, '*');

        by = y;
    }
}

fn f(y: i32, x: i32, n: i32) -> Vec<(i32, i32)> {
    if n == 3 {
        vec![(y, x),
            (y + 1, x - 1), (y + 1, x + 1),
            (y + 2, x - 2), (y + 2, x - 1), (y + 2, x), (y + 2, x + 1), (y + 2, x + 2)]
    } else {
        vec![f(y, x, n / 2),
            f(y + n / 2, x - n / 2, n / 2),
            f(y + n / 2, x + n / 2, n / 2)].into_iter()
                                           .flat_map(|x| x.into_iter())
                                           .collect()
    }
}
