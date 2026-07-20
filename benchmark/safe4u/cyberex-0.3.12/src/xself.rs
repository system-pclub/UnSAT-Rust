use std::pin::Pin;

/**

   # Safety

   This function is unsafe. You must guarantee that you will never move the data out of the
   mutable reference you receive when you call this function, so that the invariants on the Pin type can be upheld.
*/
pub unsafe fn self_mut_from_pinbox<T>(p: &mut Pin<Box<T>>) -> &mut T {
    p.as_mut().get_unchecked_mut()
}
pub fn self_from_pinbox<T>(p: &Pin<Box<T>>) -> &T {
    p.as_ref().get_ref()
}
