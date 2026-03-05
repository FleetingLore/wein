use std::env::args;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    println!("args = {:?}", args);
}
