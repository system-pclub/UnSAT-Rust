use gs11n::decoder::Decoder;
use gs11n::encoder::Encoder;

#[test]
fn vector_serialization_test() {
    let vec_f32 = vec![1.0f32, 2.1f32, 3.2f32, 4.3f32];
    let encoder = Encoder::from(&vec_f32);
    let encode_result = encoder.encode();
    let decoder = Decoder::from_data(encode_result.as_slice());
    let decode_f32: Vec<f32> = decoder.decode().unwrap();
    assert_eq!(decode_f32, vec_f32);

    let vec_i32 = vec![1, 10, 100, 1000];
    let encoder = Encoder::from(&vec_i32);
    let encode_result = encoder.encode();
    let decoder = Decoder::from_data(encode_result.as_slice());
    let decode_i32: Vec<i32> = decoder.decode().unwrap();
    assert_eq!(decode_i32, vec_i32);
}
