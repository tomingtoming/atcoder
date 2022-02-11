fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let a: usize = buf.trim().parse().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut split = buf.trim().split_whitespace();
    let b: usize = split.next().unwrap().parse().unwrap();
    let c: usize = split.next().unwrap().parse().unwrap();

    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let s = buf.trim().to_string();

    println!("{} {}", a + b + c, s);
}
