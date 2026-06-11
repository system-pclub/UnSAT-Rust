use gs11n::decoder::{decode_field, decode_wired_id, DecodeContext, DecodeError, Decoder};
use gs11n::encoder::{encode_field, size_of_field, Encoder};
use gs11n::meta_data::Metadata;
use gs11n::wire_type::WireType;
use gs11n::{DeSerialization, Serialization, WireTypeTrait};

#[derive(Default)]
struct Foo {
    // #[serialized(0)]
    f_0: i16,
    // #[serialized(1)]
    f_1: f32,
    // #[serialized(2)]
    f_2: u16,
    // #[serialized(3)]
    f_3: Vec<i32>,
    // #[serialized(4)]
    f_4: Option<f32>,
    // #[serialized(30)]
    f_30: u16,
    // #[serialized(31)]
    f_31: u32,
}

impl WireTypeTrait for Foo {}

impl Serialization for Foo {
    fn encode(&self, ptr: &mut *mut u8, meta_data: &mut Metadata) {
        encode_field(0, &self.f_0, ptr, meta_data.get(0));
        encode_field(1, &self.f_1, ptr, meta_data.get(1));
        encode_field(2, &self.f_2, ptr, meta_data.get(2));
        encode_field(3, &self.f_3, ptr, meta_data.get(3));
        encode_field(4, &self.f_4, ptr, meta_data.get(4));
        encode_field(30, &self.f_30, ptr, meta_data.get(30));
        encode_field(31, &self.f_31, ptr, meta_data.get(31));
    }

    fn record(&self, meta_data: &mut gs11n::meta_data::Metadata) {
        self.f_0.record(meta_data.get(0));
        self.f_1.record(meta_data.get(1));
        self.f_2.record(meta_data.get(2));
        self.f_3.record(meta_data.get(3));
        self.f_4.record(meta_data.get(4));
        self.f_30.record(meta_data.get(30));
        self.f_31.record(meta_data.get(31));

        let mut size = size_of_field::<i16>(0, meta_data.get(0));
        size += size_of_field::<f32>(1, meta_data.get(1));
        size += size_of_field::<u16>(2, meta_data.get(2));
        size += size_of_field::<Vec<i32>>(3, meta_data.get(3));
        size += size_of_field::<Option<f32>>(4, meta_data.get(4));
        size += size_of_field::<u16>(30, meta_data.get(30));
        size += size_of_field::<u32>(31, meta_data.get(31));

        meta_data.size = size as usize;
    }
}

impl DeSerialization for Foo {
    fn decode(ptr: &mut *const u8, ctx: &DecodeContext) -> Result<Self, DecodeError> {
        let mut v = Self::default();
        while (*ptr).lt(&ctx.bounds_checker.get_bound()) {
            let (id, wire_type) = decode_wired_id(ptr, ctx)?;
            let is_prefab = wire_type == WireType::Prefab;
            match id {
                0 => v.f_0 = decode_field(ptr, ctx, is_prefab)?,
                1 => v.f_1 = decode_field(ptr, ctx, is_prefab)?,
                2 => v.f_2 = decode_field(ptr, ctx, is_prefab)?,
                3 => v.f_3 = decode_field(ptr, ctx, is_prefab)?,
                4 => v.f_4 = decode_field(ptr, ctx, is_prefab)?,
                30 => v.f_30 = decode_field(ptr, ctx, is_prefab)?,
                31 => v.f_31 = decode_field(ptr, ctx, is_prefab)?,
                _ => {
                    ctx.skip(ptr, wire_type)?;
                }
            }
        }
        Result::Ok(v)
    }
}

#[test]
fn struct_serialization_test() {
    let foo = Foo {
        f_0: -1,
        f_1: 0.1,
        f_2: 0x80,
        f_3: vec![1, 10, 100, 1000],
        f_4: Some(std::f32::consts::PI),
        f_30: 0,
        f_31: 0x80,
    };
    let encoder = Encoder::from(&foo);
    let real = encoder.encode();
    let expected: Vec<u8> = vec![
        // f_0
        0b110_00000,
        0x1,
        // f_1
        0b010_00001,
        0xCD,
        0xCC,
        0xCC,
        0x3D,
        // f_2
        0b110_00010,
        0x80,
        0x1,
        // f_3
        0b111_00011,
        0x7,
        0x4,
        2,
        20,
        200,
        1,
        208,
        15,
        // f_4
        0b111_00100,
        0x5,
        0x4,
        0xDB, 0xF, 0x49, 0x40,
        // f_30
        0b110_11110,
        0x0,
        // f_31
        0b110_11111,
        0x1,
        0x80,
        0x1,
    ];
    assert_eq!(real, expected);

    let decoder = Decoder::from_data(real.as_slice());
    let foo2 = decoder.decode::<Foo>().unwrap();
    assert_eq!(foo.f_0, foo2.f_0);
    assert_eq!(foo.f_1, foo2.f_1);
    assert_eq!(foo.f_2, foo2.f_2);
    assert_eq!(foo.f_3, foo2.f_3);
    assert_eq!(foo.f_4, foo2.f_4);
    assert_eq!(foo.f_30, foo2.f_30);
    assert_eq!(foo.f_31, foo2.f_31);
}

#[test]
fn struct_reflection_test() {}
