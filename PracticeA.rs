use std::io;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();

    let mut a_s = String::new();
    stdin.read_line(&mut a_s).ok();
    let a: i32 = a_s.trim().parse().unwrap();

    let mut b_s = String::new();
    stdin.read_line(&mut b_s).ok();
    let mut it = b_s.split_whitespace().map(|n| i32::from_str(n).unwrap());
    let (b, c) = (it.next().unwrap(), it.next().unwrap());

    let mut s = String::new();
    stdin.read_line(&mut s).ok();
    
    println!("{} {}", a+b+c, s);
}
