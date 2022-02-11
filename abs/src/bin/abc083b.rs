use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }
    let mut grand_sum = 0;
    for i in 1..n + 1 {
        let sum: u32 = i.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum();
        if a <= sum && sum <= b {
            grand_sum += i;
        }
    }
    println!("{}", grand_sum);
}
