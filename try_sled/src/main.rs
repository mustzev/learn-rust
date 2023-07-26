use crate::{
    sled::{basic, merge_operator},
    sled_structured::upsert,
};

mod sled;
mod sled_structured;

fn main() {
    println!("Try sled!");
    let _ = basic();
    let _ = merge_operator();
    upsert();
}
