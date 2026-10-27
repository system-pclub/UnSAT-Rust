/// Utility function to cast any sized type into slice of bytes
/// This is mostly for debugging purposes
/// # Safety
/// This is safe since returned slice is readonly (as long as you do not modify thing it is pointing into), and returned slice is lifetime-bound to
/// the original reference.
/// # Examples
/// Let us see how 42 is represented:
/// ```
/// # use borrowck_sacrifices::unsafe_casts::any_as_u8_slice;
/// let thing = 42i32;
/// let u8slice = any_as_u8_slice(&thing);
/// println!("{:?}", u8slice);
/// ```
/// Make sure we can not shoot ourselves in the foot:
/// ```compile_fail
/// # use borrowck_sacrifices::unsafe_casts::any_as_u8_slice;
/// 
/// let u8slice = {
///    let thing = 42i32;// this variable exists only in inner scope
///    any_as_u8_slice(&thing);
///    }
/// 
/// println!("{:?}", u8slice); //accessing its byte view past its lifetime will not work
/// ```
pub fn any_as_u8_slice<'a,T: Sized>(p: &'a T) -> &'a [u8] {
    unsafe{
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
    }
}

/// Same as any_as_u8_slice but with mutable references. This one is obviously unsafe, as you can
/// modify any byte of the "transformed" type with this. You have been warned.
pub unsafe fn any_as_u8_slice_mut<'a,T: Sized>(p: &'a mut T) -> &'a mut [u8] {
    {
    ::core::slice::from_raw_parts_mut((p as *mut T) as *mut u8, ::core::mem::size_of::<T>())
    }
}

/// Detach lifetime of mutable reference passed in, and make it match the lifetime of the target reference.
/// This operation is perfectly safe if and only if used in an appropriate context (such as iterators).
/// Using this to get around borrow checker in other contexts will lead to nukes getting launched.
pub unsafe fn lifetime_detach<'a, 'b, T>(src: &'a mut T) -> &'b mut T
where
    'b: 'a,
{
    let mut_ptr = src as *mut T;
    mut_ptr.as_mut().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::unsafe_casts::*;

    struct SillyIter<'a> {
        data: &'a mut [i32],
        index: usize,
    }
    impl<'a> Iterator for SillyIter<'a> {
        type Item = &'a mut i32;

        fn next(&mut self) -> Option<Self::Item> {
            let rv = self.data.get_mut(self.index)?;            
            self.index += 1;
            // Here we rebind the lifetime of rv to match the Iterator's own lifetime. 
            // this is perfectly safe, but borrowck is too stupid to know that.
            Some(unsafe{lifetime_detach(rv)})
        }
    }
    
    #[test]
    pub fn detach_mut() {
        let mut data = [0, 1, 2, 3, 4, 5];
        {
            let iter = SillyIter {
                data: &mut data,
                index: 0,
            };
            for i in iter {
                *i = 0;
            }
        }
        let s: i32 = data.iter().sum();
        assert_eq!(s, 0);
    }
}
