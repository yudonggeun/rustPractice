use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let repeat = input.trim().parse().unwrap();

    (1..=repeat).for_each(|i| {
        println!("{}", "*".repeat(i));
    });
}