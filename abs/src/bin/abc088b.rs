use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an:  [usize; n],
    }
    an.sort();
    an.reverse();
    let mut alice_point = 0;
    let mut bob_point = 0;
    for (i, val) in an.iter().enumerate() {
        if i % 2 == 0 {
            alice_point += val;
        } else {
            bob_point += val;
        }
    }
    println!("{}", alice_point - &bob_point);
}
