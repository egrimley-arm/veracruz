use std::fs;

fn main() {
    let s1 = "This is some test to be ROT13-encoded. ";
    let s2 = "And here is some text.";
    println!("{:?}", fs::write("/sock", s1));
    println!("{:?}", fs::write("/sock", s2));
    println!("{}", String::from_utf8_lossy(&fs::read("/sock").unwrap()));
}
