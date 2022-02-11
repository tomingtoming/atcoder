use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }
    let mut count = 0;
    for coin500 in 0..a + 1 {
        for coin100 in 0..b + 1 {
            for coin50 in 0..c + 1 {
                if 500 * coin500 + 100 * coin100 + 50 * coin50 == x {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
