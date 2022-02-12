use proconio::input;

fn main() {
    input! {
        n: usize,
        mut dn: [usize; n],
    }
    dn.sort();
    dn.dedup();
    println!("{}", dn.len());
}
