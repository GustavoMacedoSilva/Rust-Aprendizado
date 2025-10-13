fn main() {
    let s: &str = "teste";
    let s1: String = s.chars().rev().collect();
    println!("{}", s1);
}