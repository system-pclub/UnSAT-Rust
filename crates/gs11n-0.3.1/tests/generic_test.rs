use gs11n::decoder::Decoder;
use gs11n::encoder::Encoder;
use gs11n::{DeSerialization, Serialization, WireTypeTrait};

#[derive(PartialEq, Debug)]
struct Position<T: Serialization + DeSerialization + Default> {
    // #[serialized(0)]
    x: T,
    // #[serialized(1)]
    y: T,
}

impl<T: Serialization + DeSerialization + Default> Default for Position<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T: Serialization + DeSerialization + Default> WireTypeTrait for Position<T> {}

impl<T: Serialization + DeSerialization + Default> gs11n::Serialization for Position<T> {
    fn encode(&self, ptr: &mut *mut u8, meta_data: &mut gs11n::meta_data::Metadata) {
        gs11n::encoder::encode_field(0, &self.x, ptr, meta_data.get(0));
        gs11n::encoder::encode_field(1, &self.y, ptr, meta_data.get(1));
    }
    fn record(&self, meta_data: &mut gs11n::meta_data::Metadata) {
        self.x.record(meta_data.get(0));
        self.y.record(meta_data.get(1));
        let mut size = gs11n::encoder::size_of_field::<T>(0, meta_data.get(0));
        size += gs11n::encoder::size_of_field::<T>(1, meta_data.get(1));
        meta_data.size = size;
    }
}

impl<T: Serialization + DeSerialization + Default> gs11n::DeSerialization for Position<T> {
    fn decode(
        ptr: &mut *const u8,
        ctx: &gs11n::serialization::decoder::DecodeContext,
    ) -> Result<Self, gs11n::decoder::DecodeError> {
        use gs11n::decoder::decode_wired_id;
        use gs11n::wire_type::WireType;
        let mut v = Self::default();
        while (*ptr).lt(&ctx.bounds_checker.get_bound()) {
            let (id, wire_type) = decode_wired_id(ptr, ctx)?;
            let is_prefab = wire_type == WireType::Prefab;
            match id {
                0 => v.x = gs11n::decoder::decode_field(ptr, ctx, is_prefab)?,
                1 => v.y = gs11n::decoder::decode_field(ptr, ctx, is_prefab)?,
                _ => {
                    ctx.skip(ptr, wire_type)?;
                }
            }
        }
        Result::Ok(v)
    }
}

#[test]
fn generic_test() {
    let position: Position<i32> = Position { x: 1, y: -1 };

    let encoder = Encoder::from(&position);
    let buffer = encoder.encode();
    assert_eq!(buffer, vec![0b110_00000, 0x2, 0b110_00001, 0x1,]);

    let decoder = Decoder::from_data(buffer.as_slice());
    let position2: Position<i32> = decoder.decode().unwrap();
    assert_eq!(position2, position);
}
