use std::io::stdin;

fn main() {
    let stdin = stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let mut indices: Vec<i8> = vec![-1; 26];
    for (i, ch) in buf.trim().chars().enumerate() {
        if indices[char2idx(ch)] == -1 {
            indices[char2idx(ch)] = i as i8;
        }
    }

    let result: Vec<String> = indices.iter()
                                    .map(|i| i.to_string())
                                    .collect();
    
    println!("{}", result.join(" "));
}

fn char2idx(ch: char) -> usize {
    ch as usize - 'a' as usize
}
