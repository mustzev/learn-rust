// This example demonstrates how to work with structured
// keys and values without paying expensive (de)serialization
// costs.

use {
    byteorder::{BigEndian, LittleEndian},
    sled::{Db, IVec, Result},
    std::collections::HashMap,
    std::str::from_utf8,
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

            let layout: LayoutVerified<&mut [u8], Value> =
                LayoutVerified::new_unaligned(&mut *backing_bytes)
                    .expect("bytes do not fit schema");

            let value: &mut Value = layout.into_mut();

            let new_count = value.count.get() + 1;

            println!("incrementing count to {}", new_count);

            value.count.set(new_count);

            Some(backing_bytes)
        } else {
            println!("setting count to 0");

            Some(IVec::from(
                Value {
                    count: U64::new(0),
                    whatever: [0; 16],
                }
                .as_bytes(),
            ))
        }
    })?;

    Ok(())
}

#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
struct CatValue {
    favorite_number: U64<LittleEndian>,
    battles_won: U64<LittleEndian>,
}

#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
struct DogValue {
    woof_count: U32<LittleEndian>,
    postal_code: U16<LittleEndian>,
}

pub fn variable_lengths(db: &Db) -> Result<()> {
    let dogs = db.open_tree(b"dogs")?;

    let mut dog2000_value = vec![];
    dog2000_value.extend_from_slice(b"science zone");
    dog2000_value.extend_from_slice(
        DogValue {
            woof_count: U32::new(666),
            postal_code: U16::new(42),
        }
        .as_bytes(),
    );
    dogs.insert("dog2000", dog2000_value)?;

    let mut zed_pup_value = vec![];
    zed_pup_value.extend_from_slice(b"bowling alley");
    zed_pup_value.extend_from_slice(
        DogValue {
            woof_count: U32::new(32113231),
            postal_code: U16::new(0),
        }
        .as_bytes(),
    );
    dogs.insert("zed pup", zed_pup_value)?;

    let mut klaus_value = vec![];
    klaus_value.extend_from_slice(b"barf shop");
    klaus_value.extend_from_slice(
        DogValue {
            woof_count: U32::new(0),
            postal_code: U16::new(12045),
        }
        .as_bytes(),
    );
    dogs.insert("klaus", klaus_value)?;

    let cats = db.open_tree(b"cats")?;

    let mut laser_cat_value = vec![];
    laser_cat_value.extend_from_slice(
        CatValue {
            favorite_number: U64::new(11),
            battles_won: U64::new(321231321),
        }
        .as_bytes(),
    );
    laser_cat_value.extend_from_slice(b"science zone");
    cats.insert("laser cat", laser_cat_value)?;

    let mut pulsar_cat_value = vec![];
    pulsar_cat_value.extend_from_slice(
        CatValue {
            favorite_number: U64::new(11),
            battles_won: U64::new(321231321),
        }
        .as_bytes(),
    );
    pulsar_cat_value.extend_from_slice(b"science zone");
    cats.insert("pulsar cat", pulsar_cat_value)?;

    let mut fluffy_value = vec![];
    fluffy_value.extend_from_slice(
        CatValue {
            favorite_number: U64::new(11),
            battles_won: U64::new(321231321),
        }
        .as_bytes(),
    );
    fluffy_value.extend_from_slice(b"bowling alley");
    cats.insert("fluffy", fluffy_value)?;

    Ok(())
}

pub fn hash_join(db: &Db) -> Result<()> {
    let cats = db.open_tree(b"cats")?;
    let dogs = db.open_tree(b"dogs")?;

    let mut join = HashMap::new();

    for name_value_res in &cats {
        // cats are stored as name -> favorite_number + battles_won + home name
        // variable bytes
        let (name, value_bytes) = name_value_res?;
        let (_, home_name): (LayoutVerified<&[u8], CatValue>, &[u8]) =
            LayoutVerified::new_from_prefix(&*value_bytes).unwrap();
        let (ref mut cat_names, _dog_names) =
            join.entry(home_name.to_vec()).or_insert((vec![], vec![]));
        cat_names.push(from_utf8(&*name).unwrap().to_string());
    }

    for name_value_res in &dogs {
        // dogs are stored as name -> home name variable bytes + woof count +
        // postal code
        let (name, value_bytes) = name_value_res?;

        // note that this is reversed from the cat example above, where
        // the variable bytes are at the other end of the value, and are
        // extracted using new_from_prefix instead of new_from_suffix.
        let (home_name, _dog_value): (_, LayoutVerified<&[u8], DogValue>) =
            LayoutVerified::new_from_suffix(&*value_bytes).unwrap();

        if let Some((_cat_names, ref mut dog_names)) = join.get_mut(home_name) {
            dog_names.push(from_utf8(&*name).unwrap().to_string());
        }
    }

    for (home, (cats, dogs)) in join {
        println!(
            "the cats {:?} and the dogs {:?} live in the same home of {}",
            cats,
            dogs,
            from_utf8(&home).unwrap()
        );
    }

    Ok(())
}
