use std::io;

fn main() {
    println!("Hey, we getting input here!");

    let mut answer: String = String::new();

    io::stdin().read_line(&mut answer)
        .expect("Yo man, somthing's wrong G");

    println!("Hey man, you typed in: {}", answer);
}
