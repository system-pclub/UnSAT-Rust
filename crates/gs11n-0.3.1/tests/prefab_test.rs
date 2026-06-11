use gs11n::decoder::{DecodeContext, DecodeError, Decoder};
use gs11n::encoder::Encoder;
use gs11n::prefab_loader::PrefabLoader;
use gs11n::serialization::wire_type::NonPrefabWireType;
use gs11n::utils::SimplePrefab;
use rustc_hash::FxHashMap;

#[derive(Default)]
struct Foo {
    v: Vec<u32>,
}

impl gs11n::WireTypeTrait for Foo {}

impl gs11n::Serialization for Foo {
    fn encode(&self, ptr: &mut *mut u8, meta_data: &mut gs11n::meta_data::Metadata) {
        gs11n::encoder::encode_field(0, &self.v, ptr, meta_data.get(0));
    }
    fn record(&self, meta_data: &mut gs11n::meta_data::Metadata) {
        self.v.record(meta_data.get(0));
        let size = gs11n::encoder::size_of_field::<Vec<u32>>(0, meta_data.get(0));
        meta_data.size = size;
    }
}

impl gs11n::DeSerialization for Foo {
    fn decode(
        ptr: &mut *const u8,
        ctx: &gs11n::decoder::DecodeContext,
    ) -> Result<Self, gs11n::decoder::DecodeError> {
        use gs11n::decoder::decode_wired_id;
        use gs11n::wire_type::WireType;
        let mut v = Self::default();
        while (*ptr).lt(&ctx.bounds_checker.get_bound()) {
            let (id, wire_type) = decode_wired_id(ptr, ctx)?;
            let is_prefab = wire_type == WireType::Prefab;
            match id {
                0 => v.v = gs11n::decoder::decode_field(ptr, ctx, is_prefab)?,
                _ => {
                    ctx.skip(ptr, wire_type)?;
                }
            }
        }
        Result::Ok(v)
    }
}

#[derive(Default)]
pub struct FooPrefab {
    v: SimplePrefab,
}

impl gs11n::WireTypeTrait for FooPrefab {}

impl gs11n::Serialization for FooPrefab {
    fn encode(&self, ptr: &mut *mut u8, meta_data: &mut gs11n::meta_data::Metadata) {
        gs11n::encoder::encode_field(0, &self.v, ptr, meta_data.get(0));
    }
    fn record(&self, meta_data: &mut gs11n::meta_data::Metadata) {
        self.v.record(meta_data.get(0));
        let size = gs11n::encoder::size_of_field::<SimplePrefab>(0, meta_data.get(0));
        meta_data.size = size;
    }
}

impl gs11n::DeSerialization for FooPrefab {
    fn decode(
        ptr: &mut *const u8,
        ctx: &gs11n::decoder::DecodeContext,
    ) -> Result<Self, gs11n::decoder::DecodeError> {
        use gs11n::decoder::decode_wired_id;
        use gs11n::wire_type::WireType;
        let mut prefab = Self::default();
        while (*ptr).lt(&ctx.bounds_checker.get_bound()) {
            let (id, wire_type) = decode_wired_id(ptr, ctx)?;
            let is_prefab = wire_type == WireType::Prefab;
            match id {
                0 => prefab.v = gs11n::decoder::decode_field(ptr, ctx, is_prefab)?,
                _ => {
                    ctx.skip(ptr, wire_type)?;
                }
            }
        }
        Result::Ok(prefab)
    }
}

struct TestPrefabLoader {
    prefabs: FxHashMap<u64, Vec<u8>>,
}

impl PrefabLoader for TestPrefabLoader {
    fn skip_wire_type(&self) -> NonPrefabWireType {
        NonPrefabWireType::Bit64
    }

    fn handle_prefab(
        &self,
        ptr: &mut *const u8,
        ctx: &DecodeContext,
    ) -> Result<&[u8], DecodeError> {
        use gs11n::DeSerialization;
        let id = u64::decode(ptr, ctx)?;
        match self.prefabs.get(&id) {
            None => Result::Err(DecodeError::PrefabNotExist),
            Some(prefab) => Result::Ok(prefab.as_slice()),
        }
    }
}

#[test]
fn prefab_test() {
    let mut prefab_loader = TestPrefabLoader {
        prefabs: FxHashMap::default(),
    };

    let prefab_content = vec![1, 2, 3];
    let encoder = Encoder::from(&prefab_content);
    let prefabs = encoder.encode();

    prefab_loader.prefabs.insert(1, prefabs);

    let prefab = FooPrefab {
        v: SimplePrefab::new(1),
    };

    let encoder = Encoder::from(&prefab);
    let result = encoder.encode();
    assert_eq!(result, vec![0b101_00000, 1,]);

    let decoder = Decoder::from_data_with_preloader(result.as_slice(), &prefab_loader);
    let foo = decoder.decode::<Foo>().unwrap();
    assert_eq!(foo.v, prefab_content);
}
