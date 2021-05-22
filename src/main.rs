use derive_more::From;
#[derive(From)]
struct Chicken<'a> {
    leg: &'a str,
}

fn main() {
    println!("Hello, world!");
}
