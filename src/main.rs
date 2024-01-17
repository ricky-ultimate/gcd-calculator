use std::io;
use std::io::Write;

fn main() {
    print!("Enter first number: ");
    io::stdout().flush().unwrap();

    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Invalid input");
    let num1: u16 = num1.trim().parse().expect("Eroor");

    print!("Enter second number: ");
    io::stdout().flush().unwrap();

    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Inavalid input");
    let num2: u16 = num2.trim().parse().expect("Error");

    println!("");

    let a = num1.max(num2);
    let b = num1.min(num2);

    let result = gcd(a, b);

    println!("");

    println!("gcd({a},{b}) = {result}")
}

fn gcd(mut a: u16, mut b: u16) -> u16 {
    if b == 0 {
        a
    } else {
        loop {
            let q = a/b;
            let r = a % b;
            println!("gcd({},{}) => {} = {} x {} + {} ",a, b, a, b, q, r);
            if r == 0 {
                return b;
            }
            a = b;
            b = r;
        }
    }
}
