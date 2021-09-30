use std::fs;

fn main() {
    println!("{:?}", fs::write("/does-not-exist", "DATA"));
    println!("{:?}", fs::write("/sock", "DATA"));
}
