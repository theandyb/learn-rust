fn main() {
    let mut x1: i32 = 3;
    let mut y: i32 = 2;
    let mut x2: i32 = 5;
    let mut sum: i32 = 0;

    while y < 4000000 {
        sum += y;
        y = x1 + x2;
        x1 = x2 + y;
        x2 = x1 + y;
    } 

    println!("The answer is {}", sum);
}
