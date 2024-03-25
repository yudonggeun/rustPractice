use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let tmp: Vec<&str> = input.trim().split(" ").collect();

    let num1: i32 = tmp[0].parse::<i32>().unwrap();
    let num2: i32 = tmp[1].parse::<i32>().unwrap();

    if num1 > num2 {
        println!(">");
    }

    else if num1 < num2 {
        println!("<");
    }

    else {
        println!("==");
    }
}
