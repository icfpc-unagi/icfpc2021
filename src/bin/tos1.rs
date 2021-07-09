use icfpc2021::{mat, util::SetMinMax as _};

fn main() {
    let mut x: i32 = 3;
    dbg!(x);
    dbg!(x.setmin(5));
    dbg!(x);
    dbg!(x.setmin(2));
    dbg!(x);
    dbg!(mat![[1, 2], [3, 4]]);
    dbg!(mat![0; 2; 3]);
}
