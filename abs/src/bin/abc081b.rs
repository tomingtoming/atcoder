use proconio::input;

fn main() {
    input! {
        n: u8,
        an: [u32; n],
    }
    for i in 1..32 {
        let div = 2u32.pow(i);
        if !an.iter().all(|a| a % div == 0) {
            println!("{}", i - 1);
            break;
        }
    }
}
