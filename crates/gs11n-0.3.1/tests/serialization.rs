use gs11n::decoder::Decoder;
use gs11n::meta_data::Metadata;
use gs11n::signed::{UnZigZag, ZigZag};
use gs11n::swap_bytes::SwapBytes;
use gs11n::unsigned::EncodeSize;
use gs11n::{DeSerialization, Serialization};

#[test]
fn floating_swap_bytes_test() {
    let pi: f32 = std::f32::consts::PI;
    assert_eq!(pi, pi.swap_bytes().swap_bytes());

    let pi: f64 = std::f64::consts::PI;
    assert_eq!(pi, pi.swap_bytes().swap_bytes());
}

#[test]
fn zigzag_test() {
    let i: i32 = 0;
    assert_eq!(0, i.zigzag());
    assert_eq!(i, i.zigzag().unzigzag());
    let i: i8 = -1;
    assert_eq!(1, i.zigzag());
    assert_eq!(i, i.zigzag().unzigzag());
    let i: i16 = 1;
    assert_eq!(2, i.zigzag());
    assert_eq!(i, i.zigzag().unzigzag());
    let i: isize = -2;
    assert_eq!(3, i.zigzag());
    assert_eq!(i, i.zigzag().unzigzag());
    let i: i32 = 2147483647;
    assert_eq!(4294967294, i.zigzag());
    assert_eq!(i, i.zigzag().unzigzag());
    let i: i64 = -2147483648;
    assert_eq!(4294967295, i.zigzag());
    assert_eq!(i, i.zigzag().unzigzag());
}

#[test]
fn integer_test() {
    let mut meta = Metadata::default();

    let mut expected: [u8; 41] = [0; 41];
    let mut real: [u8; 41] = [0; 41];
    let mut ptr = &mut real as *mut u8;
    let ptr = &mut ptr;

    let n1 = 0u32;
    expected[0] = 0x0u8;
    n1.encode(ptr, &mut meta);
    assert_eq!(n1.varint_size(), 1);

    let n2 = 0xffffffffu32;
    expected[1..=5].copy_from_slice(&[0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xfu8]);
    n2.encode(ptr, &mut meta);
    assert_eq!(n2.varint_size(), 5);

    let n3 = 0b0111_1111_0111_1111_0111_1111_0111_1111_u32;
    expected[6..=10].copy_from_slice(&[0xFF, 0xFE, 0xFD, 0xFB, 0x7]);
    n3.encode(ptr, &mut meta);
    assert_eq!(n3.varint_size(), 5);

    let n4 = 0b0111_1111_0111_1111_u16;
    expected[11..=13].copy_from_slice(&[0xFF, 0xFE, 0x1]);
    n4.encode(ptr, &mut meta);
    assert_eq!(n4.varint_size(), 3);

    let n5 = -1;
    expected[14] = 0x1;
    n5.encode(ptr, &mut meta);
    // assert_eq!(n5.varint_size(), 1);

    let n6 = 1;
    expected[15] = 0x2;
    n6.encode(ptr, &mut meta);
    // assert_eq!(n6.varint_size(), 1);

    let n7 = 0x79u8;
    expected[16] = 0x79;
    n7.encode(ptr, &mut meta);
    assert_eq!(n7.varint_size(), 1);

    let n8 = 0x80u8;
    expected[17..=18].copy_from_slice(&[0x80, 0x1]);
    n8.encode(ptr, &mut meta);
    assert_eq!(n8.varint_size(), 2);

    let n9: f32 = std::f32::consts::PI;
    expected[19..=22].copy_from_slice(&[0xDB, 0xF, 0x49, 0x40]);
    n9.encode(ptr, &mut meta);
    // assert_eq!(n9.varint_size(), 4);

    let n10: f64 = std::f64::consts::PI;
    expected[23..=30].copy_from_slice(&[0x18, 0x2D, 0x44, 0x54, 0xFB, 0x21, 0x9, 0x40]);
    n10.encode(ptr, &mut meta);
    // assert_eq!(n10.varint_size(), 8);

    let n11: char = '💖';
    expected[31..=34].copy_from_slice(&[0x96, 0xF4, 0x1, 0x0]);
    n11.encode(ptr, &mut meta);
    // assert_eq!(n11.varint_size(), 4);

    assert_eq!(real, expected);

    let decoder = Decoder::from_data(&real);
    let decode_ctx = decoder.get_context();

    let mut ptr: *const u8 = &real as *const u8;

    let r1 = u32::decode(&mut ptr, decode_ctx).unwrap();
    assert_eq!(r1, n1);

    let r2 = u32::decode(&mut ptr, decode_ctx).unwrap();
    assert_eq!(r2, n2);

    let r3 = u32::decode(&mut ptr, decode_ctx).unwrap();
    assert_eq!(r3, n3);

    let r4 = u16::decode(&mut ptr, decode_ctx).unwrap();
    assert_eq!(r4, n4);

    let r5 = i32::decode(&mut ptr, decode_ctx).unwrap();
    assert_eq!(r5, n5);

    let r6 = i32::decode(&mut ptr, decode_ctx).unwrap();
    assert_eq!(r6, n6);

    let r7 = u8::decode(&mut ptr, decode_ctx).unwrap();
    assert_eq!(r7, n7);

    let r8 = u8::decode(&mut ptr, decode_ctx).unwrap();
    assert_eq!(r8, n8);

    let r9 = f32::decode(&mut ptr, decode_ctx).unwrap();
    assert_eq!(r9, n9);

    let r10 = f64::decode(&mut ptr, decode_ctx).unwrap();
    assert_eq!(r10, n10);

    let r11 = char::decode(&mut ptr, decode_ctx).unwrap();
    assert_eq!(r11, n11);
}
