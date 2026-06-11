use gs11n::decoder::Decoder;
use gs11n::encoder::Encoder;

pub trait ToString {
    fn to_string(&self) -> String;
    #[doc(hidden)]
    fn type_id(&self) -> usize;
    #[doc(hidden)]
    fn dyn_encode(&self, ptr: &mut *mut u8, meta_data: &mut gs11n::meta_data::Metadata);
    #[doc(hidden)]
    fn dyn_record(&self, meta_data: &mut gs11n::meta_data::Metadata);
}

const _DYN_METADATA_FOR_TO_STRING: () = {
    lazy_static::lazy_static! {
        static ref VTABLE : gs11n::dynamic::VTable<dyn ToString>
            =  std::sync::RwLock::new(rustc_hash::FxHashMap::default());
    }
    impl dyn ToString {
        #[doc(hidden)]
        pub fn register_type(id: usize, decoder: gs11n::dynamic::DecodeFn<dyn ToString>) {
            let mut v_table = VTABLE.write().unwrap();
            v_table.insert(id, decoder);
        }
    }

    impl gs11n::WireTypeTrait for Box<dyn ToString> {}

    impl gs11n::Serialization for Box<dyn ToString> {
        fn encode(&self, ptr: &mut *mut u8, meta_data: &mut gs11n::meta_data::Metadata) {
            self.type_id().encode(ptr, meta_data);
            self.dyn_encode(ptr, meta_data)
        }
        fn record(&self, meta_data: &mut gs11n::meta_data::Metadata) {
            self.dyn_record(meta_data)
        }
    }

    impl gs11n::DeSerialization for Box<dyn ToString> {
        fn decode(
            ptr: &mut *const u8,
            ctx: &gs11n::decoder::DecodeContext,
        ) -> Result<Self, gs11n::decoder::DecodeError> {
            let id = usize::decode(ptr, ctx)?;
            let v_table = REF_VTABLE.load(std::sync::atomic::Ordering::Relaxed);
            let v_table = unsafe { &*v_table }.read().unwrap();

            match v_table.get(&id) {
                Some(decode_fn) => {
                    let the_box = decode_fn(ptr, ctx)?;
                    Ok(the_box)
                }
                None => Err(gs11n::decoder::DecodeError::InvalidType),
            }
        }
    }

    lazy_static::lazy_static! {
         static ref REF_VTABLE: std::sync::atomic::AtomicPtr<gs11n::dynamic::VTable<dyn ToString>>
                = std::sync::atomic::AtomicPtr::new(&*VTABLE as *const gs11n::dynamic::VTable<dyn ToString> as *mut gs11n::dynamic::VTable<dyn ToString>);
    }

    pub unsafe fn sync_trait(v_table: &'static gs11n::plugin::UnsafeVTable) {
        let v_table: &'static gs11n::dynamic::VTable<dyn ToString> = std::mem::transmute(v_table);
        {
            let mut caller = v_table.write().unwrap();
            let callee = VTABLE.read().unwrap();
            for (id, decode_fn) in &*callee {
                caller.insert(*id, *decode_fn);
            }
        }
        REF_VTABLE.store(
            v_table
                as *const std::sync::RwLock<
                    rustc_hash::FxHashMap<usize, gs11n::dynamic::DecodeFn<dyn ToString>>,
                >
                as *mut std::sync::RwLock<
                    rustc_hash::FxHashMap<usize, gs11n::dynamic::DecodeFn<dyn ToString>>,
                >,
            std::sync::atomic::Ordering::Relaxed,
        );
    }

    #[ctor::ctor]
    unsafe fn register_trait() {
        let vtable: &gs11n::dynamic::VTable<dyn ToString> = &*VTABLE;
        let mut register = gs11n::plugin::REGISTERED_TRAITS.lock().unwrap();
        let trait_info = gs11n::plugin::TraitInfo {
            vtable: std::mem::transmute(vtable),
            update_fn: sync_trait,
        };
        register.insert(
            String::from(std::any::type_name::<dyn ToString>()),
            trait_info,
        );
    }
};

pub trait TypeId {
    const GS11N_TYPE_ID: usize;
}

impl TypeId for i32 {
    const GS11N_TYPE_ID: usize = 1;
}

impl TypeId for char {
    const GS11N_TYPE_ID: usize = 2;
}

impl ToString for i32 {
    fn to_string(&self) -> String {
        let mut v = *self;
        let mut s: String = String::new();
        while v != 0 {
            let i: u8 = (v % 10) as u8;
            v /= 10;
            s.push((i + b'0') as char);
        }
        let str = s.as_str();
        let s2 = str.chars().rev().collect();
        s2
    }
    fn type_id(&self) -> usize {
        Self::GS11N_TYPE_ID
    }
    fn dyn_encode(&self, ptr: &mut *mut u8, meta_data: &mut gs11n::meta_data::Metadata) {
        use gs11n::Serialization;
        self.encode(ptr, meta_data);
    }
    fn dyn_record(&self, meta_data: &mut gs11n::meta_data::Metadata) {
        use gs11n::unsigned::EncodeSize;
        use gs11n::Serialization;
        self.record(meta_data.get(0));
        meta_data.size = meta_data.get(0).size + Self::GS11N_TYPE_ID.varint_size();
    }
}

#[ctor::ctor]
fn register_i32_for_to_string() {
    <dyn ToString>::register_type(
        i32::GS11N_TYPE_ID,
        |ptr: &mut *const u8, ctx: &gs11n::decoder::DecodeContext| {
            use gs11n::DeSerialization;
            let v = i32::decode(ptr, ctx)?;
            Ok(Box::new(v))
        },
    )
}

impl ToString for char {
    fn to_string(&self) -> String {
        String::from(*self)
    }

    fn type_id(&self) -> usize {
        Self::GS11N_TYPE_ID
    }
    // gen
    fn dyn_encode(&self, ptr: &mut *mut u8, meta_data: &mut gs11n::meta_data::Metadata) {
        use gs11n::Serialization;
        self.encode(ptr, meta_data)
    }

    fn dyn_record(&self, meta_data: &mut gs11n::meta_data::Metadata) {
        use gs11n::unsigned::EncodeSize;
        use gs11n::Serialization;
        self.record(meta_data.get(0));
        meta_data.size = meta_data.get(0).size + Self::GS11N_TYPE_ID.varint_size();
    }
}

// gen
#[ctor::ctor]
fn register_type_char_for_to_string() {
    <dyn ToString>::register_type(
        char::GS11N_TYPE_ID,
        |ptr: &mut *const u8, ctx: &gs11n::decoder::DecodeContext| {
            use gs11n::DeSerialization;
            let v = char::decode(ptr, ctx)?;
            Ok(Box::new(v))
        },
    )
}

#[no_mangle]
fn get_encode_buffer() -> Vec<u8> {
    let b1: Box<dyn ToString> = Box::new(256i32);
    let encoder = Encoder::from(&b1);
    let v = encoder.encode();
    v
}

#[test]
fn dyn_test() {
    {
        let b1: Box<dyn ToString> = Box::new(256i32);
        let encoder = Encoder::from(&b1);
        let v = encoder.encode();
        let decoder = Decoder::from_data(v.as_slice());
        let b2: Box<dyn ToString> = decoder.decode().unwrap();
        assert_eq!(b2.to_string(), "256");
    }

    {
        let b1: Box<dyn ToString> = Box::new('x');
        let encoder = Encoder::from(&b1);
        let v = encoder.encode();
        let decoder = Decoder::from_data(v.as_slice());
        let b2: Box<dyn ToString> = decoder.decode().unwrap();
        assert_eq!(b2.to_string(), "x");
    }
}

#[test]
fn non_dyn_test() {
    let box_i = Box::new(1);
    let encoder = Encoder::from(&box_i);
    let v = encoder.encode();
    let decoder = Decoder::from_data(v.as_slice());
    let box_j: Box<i32> = decoder.decode().unwrap();
    assert_eq!(*box_i, *box_j);
    assert_ne!(&*box_i as *const i32, &*box_j as *const i32);
}
