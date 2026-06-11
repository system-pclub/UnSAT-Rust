pub fn get_two_mut_elems<'a, T>(x: &'a mut Vec<T>, i: usize, j: usize) -> (&'a mut T, &'a mut T) {
    let len = x.len();
    assert!(i != j);
    assert!(i != len);
    assert!(j != len);
    let ptr = x.as_mut_ptr();
    unsafe { (ptr.add(i).as_mut().unwrap(), ptr.add(j).as_mut().unwrap()) }
}
