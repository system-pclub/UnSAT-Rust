#[allow(missing_docs)]
#[cfg(feature = "unsat-poc-src-rank-select-mod-rs-328-21-rule-575-c6948a15cd")]
#[no_mangle]
pub extern "C" fn __unsat_poc_src_rank_select_mod_rs_328_21_rule_575_c6948a15cd() {
    let mut __unsat_rerun_sym_000 = 64;
    klee_ext_bind::make_symbolic!(&mut __unsat_rerun_sym_000, "__unsat_rerun_sym_000");
    use crate::RankSimple;
    use std::ops::Deref;

    struct BV {
        data: Box<[u64]>,
    }

    impl Deref for BV {
        type Target = [u64];
        fn deref(&self) -> &[u64] {
            &self.data
        }
    }

    let content = BV {
        data: vec![0u64; 1].into_boxed_slice(),
    };

    let ranks = vec![7u32, 11u32].into_boxed_slice();

    let rs = RankSimple { content, ranks };

    let _ = rs.try_rank(__unsat_rerun_sym_000);
}

