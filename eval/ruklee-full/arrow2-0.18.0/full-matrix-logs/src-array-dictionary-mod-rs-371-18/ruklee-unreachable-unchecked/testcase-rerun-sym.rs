#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-array-dictionary-mod-rs-371-18-ruklee-unreachable-unchecked-bd87df9fef")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_array_dictionary_mod_rs_371_18_ruklee_unreachable_unchecked_bd87df9fef() {
    let mut __unsat_rerun_sym_000 = 1;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    let keys = crate::array::PrimitiveArray::<i8>::from_vec(vec![-__unsat_rerun_sym_000]);
    let values = crate::array::new_empty_array(crate::datatypes::DataType::Int32);
    let data_type = crate::datatypes::DataType::Dictionary(
        crate::datatypes::IntegerType::Int8,
        Box::new(crate::datatypes::DataType::Int32),
        __unsat_rerun_sym_001,
    );

    let dict = crate::array::dictionary::DictionaryArray::<i8>::try_new(data_type, keys, values)
        .unwrap();

    let _ = dict.key_value(__unsat_rerun_sym_002);
}

