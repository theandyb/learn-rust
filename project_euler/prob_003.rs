fn prime_factorization (x: i64){
    let mut i: i64 = 2;
    let mut y: i64 = x;
    let x2 = x as f64;

    while y % i == 0 {
        y = y/i;
        print!("2 times ");
    }
    
    i = 3;

    while i <= (x2.sqrt().floor() as i64) {
        while y % i == 0 {
            y = y/i;
            print!("{} times ", i);
        }
        i+=2;
    }
    println!("one!");
}

fn main() {
    prime_factorization(600851475143);
}
