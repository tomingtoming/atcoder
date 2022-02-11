use proconio::input;

fn main() {
    input! {
        given: String,
    }
    println!("{}", given.chars().filter(|c| c.to_owned() == '1').count())
}
