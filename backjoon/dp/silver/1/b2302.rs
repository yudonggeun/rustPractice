// https://www.acmicpc.net/problem/2302
use std::io::stdin;

fn main() {
    let size: usize = read_number_line();
    let fixed_size: usize = read_number_line();

    let mut dp = vec![0; size + 1];

    dp[0] = 1;
    dp[1] = 1;

    for i in 2..=size {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    let mut front_vip = 0;
    let mut result = 1;
    for _ in 0..fixed_size {
        let vip = read_number_line();
        result = result * dp[vip - front_vip - 1];
        front_vip = vip;
    }
    result = result * dp[size - front_vip];

    println!("{}", result);
}

fn read_number_line() -> usize{
    let mut input = String::new();

    let _ = stdin().read_line(&mut input);
    input.trim().parse::<usize>().unwrap_or_else(|_| {
        println!("Invalid number format exception");
        0
    })
}