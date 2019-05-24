use rand::distributions::{Normal, Poisson, Distribution};

fn main() {
    println!("Hello, world!");

    let normal = Normal::new(2.0, 3.0); //mean 0, sd = 3
    let v = normal.sample(&mut rand::thread_rng());
    
    println!("We sampled {} from a N(2,9) distribution.", v);

    let poisson = Poisson::new(2.0);
    let v2 = poisson.sample(&mut rand::thread_rng());

    println!("We sampled {} from a Poisson(2) distribution.", v2);
}
