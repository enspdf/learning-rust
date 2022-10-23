#![deny(clippy::all)]

use intutils::addition::add;
use intutils::subtraction::sub;

fn main() {
    let added = add(1, 2);
    let subtracted = sub(1, 2);

    println!("{}", added);
    println!("{}", subtracted);
}
