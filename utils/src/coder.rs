use bincode;
use serde::{Deserialize, Serialize};

// pub fn serialize<T: ?Sized>(value: &T) -> Result<Vec<u8>> 
// where
//     T: Serialize, 


pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
    where T: Serialize,
{
    let serialized = bincode::serialize(value).unwrap();
    serialize
}


// pub fn deserialize<'a, T>(bytes: &'a [u8]) -> Result<T> 
// where
//      T: Deserialize<'a>,

pub fn my_deserialize<a', T>(bytes: &'a[u8]) -> T 
    where T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}