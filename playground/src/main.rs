mod my_funcs;
mod other_funcs;

use crate::my_funcs::add_five;
use crate::other_funcs::minus_funcs::subtract_10;

// Everything in defaulted to inmutable
fn main() {
    let x: u32 = 50;
    println!("{}", x);

    let y: u32 = add_five(x);
    println!("{}", y);

    let z: u32 = subtract_10(y);
    println!("{}", z)
}
