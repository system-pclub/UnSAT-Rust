use gs11n::decoder::Decoder;
use gs11n::dynamic::VTable;
use gs11n::plugin::{TraitRegister, REGISTERED_TRAITS};

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
        let mut register = gs11n::plugin::REGISTERED_TRAITS.lock().unwrap();
        let vtable: &VTable<dyn ToString> = &*VTABLE;
        let trait_info = gs11n::plugin::TraitInfo {
            vtable: std::mem::transmute(vtable),
            update_fn: sync_trait,
        };
        // hack, we need the trait's full name to be "dyn gs11n_cdylib_dynamic_test::ToString"
        register.insert(
            String::from("dyn gs11n_cdylib_dynamic_test::ToString"),
            trait_info,
        );
    }
};

#[test]
fn plugin_test() {
    let dylib_path = test_cdylib::build_file("tests/dynamic_test.rs");
    unsafe {
        let lib = libloading::Library::new(dylib_path).unwrap();
        let sync_traits: libloading::Symbol<fn(caller_register: &TraitRegister)> =
            lib.get(b"sync_traits").unwrap();
        {
            let register_traits = REGISTERED_TRAITS.lock().unwrap();
            sync_traits(&register_traits);
        }

        let get_encode_buffer: libloading::Symbol<fn() -> Vec<u8>> =
            lib.get(b"get_encode_buffer").unwrap();
        let encode_buffer = get_encode_buffer();

        let decoder = Decoder::from_data(encode_buffer.as_slice());
        let b: Box<dyn ToString> = decoder.decode().unwrap();
        assert_eq!(b.to_string(), "256");
    }
}
