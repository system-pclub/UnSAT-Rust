use gs11n::decoder::Decoder;
use gs11n::encoder::Encoder;
use std::collections::{BTreeMap, HashMap};

#[test]
fn map_test() {
    let mut hash_map: HashMap<u32, &str> = HashMap::default();
    hash_map.insert(1, "one");
    hash_map.insert(2, "two");
    let encoder = Encoder::from(&hash_map);
    let encode_result = encoder.encode();

    let decoder = Decoder::from_data(encode_result.as_slice());
    let decode_map: BTreeMap<u32, String> = decoder.decode().unwrap();

    assert_eq!(hash_map.len(), decode_map.len());

    assert_eq!(hash_map.get(&1).unwrap(), decode_map.get(&1).unwrap());
    assert_eq!(hash_map.get(&2).unwrap(), decode_map.get(&2).unwrap());
}
