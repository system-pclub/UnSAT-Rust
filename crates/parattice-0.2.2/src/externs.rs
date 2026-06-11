use crate::lattice::Lattice;
use crate::lattice_kmp::LatticeKMP;
use crate::parattice::PaRattice;

use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::slice;
use std::str;

#[no_mangle]
pub unsafe extern "C" fn parattice_free_string(s: *mut c_char) {
    if !s.is_null() {
        CString::from_raw(s);
    }
}

#[no_mangle]
pub unsafe extern "C" fn parattice_free_bytes(bytes: *mut u8, length: usize) {
    mem::drop(Vec::from_raw_parts(bytes, length, length));
}

#[no_mangle]
pub unsafe extern "C" fn parattice_parattice_new<'a>(
    dict: *const *const *const *const c_char,
) -> *mut PaRattice<'a> {
    let mut dict_vec = vec![];
    let mut i = 0;
    while !dict.offset(i).is_null() {
        let group: *const *const *const c_char = *dict.offset(i);
        let mut group_vec = vec![];
        let mut j = 0;
        while !group.offset(j).is_null() {
            let phrase: *const *const c_char = *group.offset(j);
            let mut phrase_vec = vec![];
            let mut k = 0;
            while !phrase.offset(k).is_null() {
                let word = *phrase.offset(k);
                let c_str = CStr::from_ptr(word);
                phrase_vec.push(str::from_utf8_unchecked(c_str.to_bytes()));
                k += 1;
            }
            group_vec.push(phrase_vec);
            j += 1
        }
        dict_vec.push(group_vec);
        i += 1;
    }
    Box::into_raw(Box::new(PaRattice::new(dict_vec)))
}

#[no_mangle]
pub unsafe extern "C" fn parattice_parattice_free(parattice: *mut PaRattice) {
    Box::from_raw(parattice);
}

#[no_mangle]
pub unsafe extern "C" fn parattice_parattice_get_lattice(
    parattice: *const PaRattice,
    words: *const *const c_char,
    length: usize,
    shrink: bool,
    max_depth: usize,
) -> *mut Lattice {
    let mut words_vec = Vec::with_capacity(length);
    for i in 0..length {
        let word = *words.add(i);
        let c_str = CStr::from_ptr(word);
        words_vec.push(str::from_utf8_unchecked(c_str.to_bytes()));
    }
    Box::into_raw(Box::new(
        (*parattice).get_lattice(&words_vec, shrink, max_depth),
    ))
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_new_from_bytes<'a>(
    data: *const u8,
    length: usize,
) -> *mut Lattice<'a> {
    Box::into_raw(Box::new(Lattice::new_from_bytes(slice::from_raw_parts(
        data, length,
    ))))
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_to_bytes(
    lattice: *const Lattice,
    length: *mut usize,
) -> *mut u8 {
    let mut bytes = (*lattice).to_bytes();
    *length = bytes.len();
    let ptr = bytes.as_mut_ptr();
    mem::forget(bytes);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_free(lattice: *mut Lattice) {
    Box::from_raw(lattice);
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_get_size(lattice: *const Lattice) -> usize {
    (*lattice).lattice.len()
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_get_required_capacity(lattice: *const Lattice) -> usize {
    (*lattice).capacity
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_dump_dot(
    lattice: *const Lattice,
    is_numbered: bool,
) -> *mut c_char {
    let s = (*lattice).dump_dot(is_numbered);
    let c_string = CString::new(s).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_get_trunk_span(
    lattice: *const Lattice,
    edge_labels: *const *const c_char,
    node_ids: *const usize,
    length: usize,
    new_edge_labels: *mut *const u8,
    new_edge_label_length: *mut usize,
    new_node_ids: *mut usize,
) -> usize {
    let mut path = Vec::with_capacity(length);
    for i in 0..length {
        let word = *edge_labels.add(i);
        let c_str = CStr::from_ptr(word);
        path.push((
            str::from_utf8_unchecked(c_str.to_bytes()),
            *node_ids.add(i),
        ));
    }
    let trunk_span = (*lattice).get_trunk_span(path);
    for (i, span) in trunk_span.iter().enumerate() {
        *new_edge_labels.add(i) = span.0.as_ptr();
        *new_edge_label_length.add(i) = span.0.len();
        *new_node_ids.add(i) = span.1;
    }
    trunk_span.len()
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_get_trunk_spans(
    lattice: *const Lattice,
    trunk_lefts: *mut usize,
    trunk_rights: *mut usize,
) {
    let trunk_spans = (*lattice).get_trunk_spans();
    for (i, (trunk_left, trunk_right)) in trunk_spans.into_iter().enumerate() {
        *trunk_lefts.add(i) = trunk_left;
        *trunk_rights.add(i) = trunk_right;
    }
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_dump_for_search_index(
    lattice: *const Lattice,
    texts: *mut *const u8,
    text_lengths: *mut usize,
    offset_starts: *mut usize,
    offset_ends: *mut usize,
    increments: *mut usize,
    lengths: *mut usize,
) -> usize {
    let search_index_nodes = (*lattice).dump_for_search_index();
    for (i, node) in search_index_nodes.iter().enumerate() {
        *texts.add(i) = node.text.as_ptr();
        *text_lengths.add(i) = node.text.len();
        *offset_starts.add(i) = node.offset.0;
        *offset_ends.add(i) = node.offset.1;
        *increments.add(i) = node.increment;
        *lengths.add(i) = node.length;
    }
    search_index_nodes.len()
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_kmp_new<'a>(
    pattern: *const *const c_char,
    length: usize,
) -> *mut LatticeKMP<'a> {
    let mut pattern_vec = Vec::with_capacity(length);
    for i in 0..length {
        let word = *pattern.add(i);
        let c_str = CStr::from_ptr(word);
        pattern_vec.push(str::from_utf8_unchecked(c_str.to_bytes()));
    }
    Box::into_raw(Box::new(LatticeKMP::new(pattern_vec)))
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_kmp_free(latticekmp: *mut LatticeKMP) {
    Box::from_raw(latticekmp);
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_kmp_search<'a>(
    latticekmp: *const LatticeKMP<'a>,
    lattice: *const Lattice<'a>,
) -> *mut Vec<Vec<(&'a str, usize)>> {
    Box::into_raw(Box::new((*latticekmp).search(&(*lattice))))
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_kmp_free_result<'a>(
    results: *mut Vec<Vec<(&'a str, usize)>>,
) {
    Box::from_raw(results);
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_kmp_results_size<'a>(
    results: *const Vec<Vec<(&'a str, usize)>>,
) -> usize {
    (*results).len()
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_kmp_result_length<'a>(
    results: *const Vec<Vec<(&'a str, usize)>>,
    index: usize,
) -> usize {
    (*results)[index].len()
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_kmp_result_nodes<'a>(
    results: *const Vec<Vec<(&'a str, usize)>>,
    index: usize,
    nodes: *mut usize,
) {
    for i in 0..(*results)[index].len() {
        *nodes.add(i) = (*results)[index][i].1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn parattice_lattice_kmp_result_edge_labels<'a>(
    results: *const Vec<Vec<(&'a str, usize)>>,
    index: usize,
    edge_labels: *mut *const u8,
    edge_label_length: *mut usize,
) {
    for i in 0..(*results)[index].len() {
        *edge_labels.add(i) = (*results)[index][i].0.as_ptr();
        *edge_label_length.add(i) = (*results)[index][i].0.len();
    }
}
