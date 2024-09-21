use std::io::{BufRead, stdin};

fn main() {

    let stdin = stdin();
    let mut lines = stdin.lock().lines();

    let repeat = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..repeat {
        let input = lines.next().unwrap().unwrap()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();

        let a = input[0];
        let b = input[1];

        let mut answer: i32 = get_mod(a, b, 10);

        if answer == 0 { answer = 10 }

        println!("{}", answer);
    }
}

fn get_mod(a: i32, b: i32, mod_val: i32) -> i32 {
    if b == 0 {
        return 1;
    } else if b == 1{
        return a % mod_val;
    }

    let half = get_mod(a, b /2, mod_val);

    return if b % 2 == 0 {
        half * half % mod_val
    } else {
        half * half * a % mod_val
    }
}