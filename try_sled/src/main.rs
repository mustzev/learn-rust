use crate::sled::{basic, merge_operator};
use crate::sled_structured::upsert;

mod sled;

fn main() {
    println!("Try sled!");
    let _ = basic();
    let _ = merge_operator();
    upsert();
}
