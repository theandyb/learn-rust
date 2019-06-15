fn main() {
    let mut ans: i32 = 3;

    let mut adder = 6;

    while adder < 1000 {
        if adder % 5 == 0 {
            adder += 3;
            continue;
        }
        ans += adder;
        adder += 3;
    }

    adder = 5;

    while adder < 1000 {
        ans += adder;
        adder += 5;
    }

    println!("{} is our answer", ans);
}
