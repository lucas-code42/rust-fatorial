use num_bigint::BigInt;
use std::io;

fn user_input() -> u64 {
    let mut i: String = String::new();
    match io::stdin().read_line(&mut i) {
        Ok(_) => {
            let input: u64 = i.trim().parse::<u64>().unwrap();
            return input;
        }
        Err(error) => println!("error: {}", error),
    }

    return 0;
}

fn factorial(n: u64) -> BigInt {
    let mut f: BigInt = BigInt::from(1);
    for i in 2..n {
        f *= i;
    }
    return f;
}

fn main() {
    let x = user_input();
    println!("{}", factorial(x));
}
