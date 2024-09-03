use std::io::stdin;

fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let count = input.trim().parse::<usize>().unwrap();

    for k in 0..count {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        
        let split_input = input.trim().split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<usize>>();

        let n = split_input[0];
        let m = split_input[1];

        let mut dp = vec![vec![0; m]; n];
        dp[n-1][m-1] = 1;
        for i in ((n-1)..(m-1)).rev() {
            dp[n-1][i] = dp[n-1][i+1] + 1;
        }

        for i in (0..n-1).rev(){
            for j in (i..(i + m -n + 1)).rev() {
                dp[i][j] = dp[i+1][j+1] + dp[i][j + 1];
            }
        }
        println!("{}", dp[0][0]);
    }
}