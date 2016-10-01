use std::io::stdin;
use std::io::prelude::BufRead;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::max;

pub fn coin2() {
    let stdin = stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let n: usize = buffer.split_whitespace().nth(0).unwrap().parse::<usize>().unwrap();
    let k: i16 = buffer.split_whitespace().nth(1).unwrap().parse::<i16>().unwrap();
    let mut coins: Vec<i16> = stdin.lock()
                            .lines()
                            .take(n)
                            .map(|x| x.unwrap().trim().parse::<i16>().unwrap())
                            .collect();

    coins.sort_by(|a,b| b.cmp(a));

    let mut memo = HashSet::<(u8, i16, i16)>::new();
    let mut pq = BinaryHeap::<(i16, u8, i16, i16)>::new();
    let mut result: Option<i16> = None;
    
    pq.push((i16::max_value(), 0, k, 0));
    while !pq.is_empty() {
        let here = pq.pop().unwrap();

        let p = -here.0;
        let i = here.1;
        let k = here.2;
        let u = here.3;

        if memo.contains(&(i, k, u)) { continue; }
        memo.insert((i, k, u));

        if Some(-p) <= result || k < 0 { continue; }

        if k == 0 {
            result = max(result, Some(-u));
        } else {
            let coin1 = coins[i as usize];
            let p1 = u + (k / coin1);
            pq.push((-p1, i, k - coin1, u + 1));

            if i as usize + 1 < coins.len() {
                let coin2 = coins[i as usize + 1];
                let p2 = u + (k / coin2);
                pq.push((-p2, i + 1, k, u));
            }
        }
    }
    println!("{:?}", match result {
        Some(x) => -x,
        None => -1,
    });
}
