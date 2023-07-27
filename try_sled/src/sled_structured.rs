// This example demonstrates how to work with structured
// keys and values without paying expensive (de)serialization
// costs.

use {
    byteorder::{BigEndian, LittleEndian},
    sled::{Db, Result, IVec},
    zerocopy::{byteorder::U64, AsBytes, FromBytes, LayoutVerified, Unaligned, U16, U32},
};

pub fn upsert(db: &Db) -> Result<()> {
    #[derive(FromBytes, AsBytes, Unaligned)]
    #[repr(C)]
    struct Key {
        a: U64<BigEndian>,
        b: U64<BigEndian>,
    }

    #[derive(FromBytes, AsBytes, Unaligned)]
    #[repr(C)]
    struct Value {
        count: U64<LittleEndian>,
        whatever: [u8; 16],
    }

    let key = Key {
        a: U64::new(21),
        b: U64::new(890),
    };

    db.update_and_fetch(key.as_bytes(), |value_opt| {
        if let Some(existing) = value_opt {
            let mut backing_bytes = IVec::from(existing);


            
        } else {

        }
    })

    Ok(())
}
