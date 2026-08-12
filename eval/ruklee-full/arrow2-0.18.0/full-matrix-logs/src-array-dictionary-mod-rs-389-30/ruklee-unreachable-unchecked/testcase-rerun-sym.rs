#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-array-dictionary-mod-rs-389-30-ruklee-unreachable-unchecked-26208c9ac8")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_array_dictionary_mod_rs_389_30_ruklee_unreachable_unchecked_26208c9ac8() {
    let mut __unsat_rerun_sym_000 = 1i8;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    let mut __unsat_rerun_sym_001 = false;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_001, "__unsat_rerun_sym_001");
    let mut __unsat_rerun_sym_002 = 0;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_002, "__unsat_rerun_sym_002");
    use crate::array::{Array, DictionaryArray, Int8Array};
    use crate::datatypes::{DataType, IntegerType};

    let keys = Int8Array::from_vec(vec![-__unsat_rerun_sym_000]);
    let values: Box<dyn Array> = Box::new(crate::array::BooleanArray::from_slice([true]));
    let data_type = DataType::Dictionary(IntegerType::Int8, Box::new(DataType::Boolean), __unsat_rerun_sym_001);

    let dict = DictionaryArray::<i8>::try_new(data_type, keys, values).unwrap();
    let _ = dict.value(__unsat_rerun_sym_002);
}

