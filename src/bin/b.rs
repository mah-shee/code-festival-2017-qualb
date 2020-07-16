#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        d: [usize; n],
        m: usize,
        t: [usize; m],
    }
    for i in 0..m {
        if !d.contains(&t[i]) {
            println!("No");
            return;
        } else {
        }
    }
    println!("Yes");
}
