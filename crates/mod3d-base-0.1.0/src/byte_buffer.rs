//a ByteBuffer
//tp ByteBuffer
/// A trait for all types that are to be used as sources of data for
/// buffers of, e.g. vertex data, indices, etc
///
/// The data is viewed by OpenGL as a pointer and byte length; these
/// methods provide access to the data in that way.
///
/// These methods are all safe - any use of the information they
/// provide may be unsafe.
pub trait ByteBuffer {
    /// Get the length of the data buffer in bytes
    fn byte_length(&self) -> usize;
    /// Borrow the data as an array of bytes
    fn borrow_bytes(&self) -> &[u8];
    /// Return a pointer to the first byte of the data contents
    fn as_u8_ptr(&self) -> *const u8;
}

//ti ByteBuffer for [T; N]
/// Implement ByteBuffer for slice of T
impl<T, const N: usize> ByteBuffer for [T; N] {
    //fp byte_length
    fn byte_length(&self) -> usize {
        std::mem::size_of::<T>() * N
    }

    //fp borrow_bytes
    fn borrow_bytes(&self) -> &[u8] {
        let len = std::mem::size_of::<T>() * self.len();
        let data = self.as_u8_ptr();
        unsafe { std::slice::from_raw_parts(data, len) }
    }

    //fp as_u8_ptr
    fn as_u8_ptr(&self) -> *const u8 {
        let data: *const T = &self[0];
        unsafe { std::mem::transmute::<_, *const u8>(data) }
    }

    //zz All done
}

//ti ByteBuffer for Vec
/// Implement ByteBuffer for Vec
impl<T> ByteBuffer for Vec<T> {
    //fp byte_length
    fn byte_length(&self) -> usize {
        std::mem::size_of::<T>() * self.len()
    }

    //fp borrow_bytes
    fn borrow_bytes(&self) -> &[u8] {
        let len = std::mem::size_of::<T>() * self.len();
        let data = self.as_u8_ptr();
        unsafe { std::slice::from_raw_parts(data, len) }
    }

    //fp as_u8_ptr
    fn as_u8_ptr(&self) -> *const u8 {
        let data: *const T = &self[0];
        unsafe { std::mem::transmute::<_, *const u8>(data) }
    }

    //zz All done
}

//ti ByteBuffer for &[T]
/// Implement ByteBuffer for &[T]
impl<T> ByteBuffer for &[T] {
    //fp byte_length
    fn byte_length(&self) -> usize {
        std::mem::size_of::<T>() * self.len()
    }

    //fp borrow_bytes
    fn borrow_bytes(&self) -> &[u8] {
        let len = std::mem::size_of::<T>() * self.len();
        let data = self.as_u8_ptr();
        unsafe { std::slice::from_raw_parts(data, len) }
    }

    //fp as_u8_ptr
    fn as_u8_ptr(&self) -> *const u8 {
        let data: *const T = self.as_ptr();
        unsafe { std::mem::transmute::<_, *const u8>(data) }
    }

    //zz All done
}
