fn main() {
    let mut ans: i32 = 0;

    let mut counter: i32 = 1;

    while counter < 1000 {
        if counter % 15 == 0 {
            ans += counter;
        } else if counter % 5 == 0 {
            ans += counter;
        } else if counter % 3 == 0 {
            ans += counter;
        }
        counter += 1;
    }

    println!("{} is the sum", ans);
}
