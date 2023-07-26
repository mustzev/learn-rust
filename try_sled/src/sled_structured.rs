// This example demonstrates how to work with structured
// keys and values without paying expensive (de)serialization
// costs.

use {
    byteorder::{BigEndian, LittleEndian},
    zerocopy::{byteorder::U64, AsBytes, FromBytes, LayoutVerified, Unaligned, U16, U32},
};

pub fn upsert() {
    println!("upsert");
}
