//a Imports
use std::cell::RefCell;

use crate::{ByteBuffer, Renderable};

//a BufferData
//tp BufferData
/// A data buffer for use with vertex data. It may be indices
/// or vertex coordinates etc.
///
/// A data buffer may contain a lot of data per vertex, such as
/// position, normal, tangent, color etc.  a `BufferView` on the data is
/// then a subset of this data - perhaps picking out just the
/// position, for example, for a set of vertices
///
/// The data buffer may, indeed, contain data for more than one object
/// - and the objects may have different data per vertex. The data
/// buffer is pretty free-form, it is a `BufferView` on the [BufferData] which
/// identifies the object it applies to, and the vertex attributes
/// required.
///
/// A data buffer may then be used by many `BufferView`s. Each `BufferView` may be
/// used by many primitives for a single model; alternatively,
/// primitives may have their own individual `BufferViews`.
///
/// A client may have one copy of the data for all the primitives and models.
pub struct BufferData<'a, R: Renderable> {
    /// Data buffer itself
    data: &'a [u8],
    /// Offset in to the data buffer for the first byte
    pub byte_offset: u32,
    /// Length of data used in the buffer
    pub byte_length: u32,
    /// The client bound to data\[byte_offset\] .. + byte_length
    ///
    /// This must be held as a [RefCell] as the [BufferData] is
    /// created early in the process, prior to any `BufferView`s using
    /// it - which then have shared references to the data - but the
    /// client is created afterwards
    rc_client: RefCell<R::Buffer>,
}

//ip Debug for BufferData
impl<'a, R: Renderable> std::fmt::Debug for BufferData<'a, R> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let (data, cont) = {
            if self.data.len() < 8 {
                (self.data, "")
            } else {
                (&self.data[0..8], "...")
            }
        };
        write!(
            fmt,
            "BufferData {{{0:?}{cont}#{4}, byte_offset:{1}, byte_length:{2}, client:{3:?}}}",
            data,
            self.byte_offset,
            self.byte_length,
            self.rc_client,
            self.data.len(),
        )
    }
}

//ip BufferData
impl<'a, R: Renderable> BufferData<'a, R> {
    //fp new
    /// Create a new [BufferData] given a buffer, offset and length; if the
    /// length is zero then the whole of the data buffer post offset
    /// is used
    ///
    /// If offset and length are both zero, then all the data is used
    pub fn new<B: ByteBuffer + ?Sized>(data: &'a B, byte_offset: u32, byte_length: u32) -> Self {
        let byte_length = {
            if byte_length == 0 {
                (data.byte_length() as u32) - byte_offset
            } else {
                byte_length
            }
        };
        let rc_client = RefCell::new(R::Buffer::default());
        let data = data.borrow_bytes();
        assert!(
            byte_offset + byte_length <= data.len() as u32,
            "Buffer is not large enough for data {} + #{} [ got {}]",
            byte_offset,
            byte_length,
            data.len()
        );
        Self {
            data,
            byte_offset,
            byte_length,
            rc_client,
        }
    }

    //mp create_client
    /// Replace the client data with one of this data
    pub fn create_client(&self, renderable: &mut R) {
        use std::ops::DerefMut;
        renderable.init_buffer_data_client(self.rc_client.borrow_mut().deref_mut(), self);
    }

    //ap borrow_client
    /// Borrow the client
    pub fn borrow_client(&self) -> std::cell::Ref<R::Buffer> {
        self.rc_client.borrow()
    }

    //mp as_ptr
    /// Get a const u8 ptr to the data itself
    pub fn as_ptr(&self) -> *const u8 {
        unsafe { self.data.as_ptr().add(self.byte_offset as usize) }
    }

    //mp as_slice
    /// Get the slice that is the data itself
    pub fn as_slice(&self) -> &[u8] {
        let start = self.byte_offset as usize;
        let end = (self.byte_offset + self.byte_length) as usize;
        self.data.get(start..end).unwrap()
    }

    //zz All done
}

//ip Display for BufferData
impl<'a, R: Renderable + ?Sized> std::fmt::Display for BufferData<'a, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let data_ptr = self.data.as_ptr();
        write!(
            f,
            "BufferData[{:?}+{}#{}]:GL({})",
            data_ptr,
            self.byte_offset,
            self.byte_length,
            self.rc_client.borrow()
        )
    }
}

//ip DefaultIndentedDisplay for BufferData
impl<'a, R: Renderable + ?Sized> indent_display::DefaultIndentedDisplay for BufferData<'a, R> {}
