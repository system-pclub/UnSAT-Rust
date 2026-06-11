//a Imports
use std::cell::RefCell;

use crate::{BufferData, BufferElementType, Renderable, VertexAttr};

//a BufferAccessor
//tp BufferAccessor
/// A subset of a `BufferData`, used for vertex attributes;
/// hence for use in a vertex attribute pointer.
///
/// A `BufferAccessor` is used for a single attribute of a set of data, such as
/// Position or Normal.
pub struct BufferAccessor<'a, R: Renderable + ?Sized> {
    /// The `BufferData` that contains the actual vertex attribute data
    pub data: &'a BufferData<'a, R>,
    /// For attributes: number of elements per vertex (1 to 4, or 4, 9 or 16)
    /// For indices: number of indices in the buffer
    pub elements_per_data: u32,
    /// The type of each element
    ///
    /// For indices this must be Int8, Int16 or Int32
    pub ele_type: BufferElementType,
    /// Offset from start of buffer to first byte of data
    pub byte_offset: u32,
    /// Stride of data in the buffer - 0 for count*sizeof(ele_type)
    /// Unused for indices
    pub stride: u32,
    /// The client bound to data\[byte_offset\] .. + byte_length
    ///
    /// This must be held as a [RefCell] as the [BufferData] is
    /// created early in the process, prior to any `BufferAccessor`s using
    /// it - which then have shared references to the daata - but the
    /// client is created afterwards
    rc_client: RefCell<R::Accessor>,
}

//ip Display for Object
impl<'a, R: Renderable> std::fmt::Debug for BufferAccessor<'a, R>
where
    R: Renderable,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "BufferAccessor{{ {:?}:{:?} #{}@{}+*{}}}",
            self.data,
            self.ele_type,
            self.elements_per_data,
            self.byte_offset,
            self.stride,
            //  self.rc_client
        )
    }
}

//ip BufferAccessor
impl<'a, R: Renderable> BufferAccessor<'a, R> {
    //fp new
    /// Create a new view of a `BufferData`
    pub fn new(
        data: &'a BufferData<'a, R>,
        count: u32, // count is number of ele_type in an attribute
        ele_type: BufferElementType,
        byte_offset: u32, // offset in bytes from start of data
        stride: u32,      /* stride between elements
                           * (0->count*sizeof(ele_type)) */
    ) -> Self {
        let rc_client = RefCell::new(R::Accessor::default());
        Self {
            data,
            elements_per_data: count,
            ele_type,
            byte_offset,
            stride,
            rc_client,
        }
    }

    //mp create_client
    /// Create the render buffer required by the BufferAccessor
    pub fn create_client(&self, attr: VertexAttr, renderable: &mut R) {
        use std::ops::DerefMut;
        renderable.init_buffer_view_client(self.rc_client.borrow_mut().deref_mut(), self, attr);
    }

    //ap borrow_client
    /// Borrow the client
    pub fn borrow_client(&self) -> std::cell::Ref<R::Accessor> {
        self.rc_client.borrow()
    }

    //zz All done
}

//ip Display for BufferAccessor
impl<'a, R: Renderable> std::fmt::Display for BufferAccessor<'a, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "BufferAccessor[{:?}#{}]\n  {}+{}+n*{}\n",
            self.ele_type, self.elements_per_data, self.data, self.byte_offset, self.stride
        )
    }
}

//ip DefaultIndentedDisplay for BufferAccessor
impl<'a, R: Renderable> indent_display::DefaultIndentedDisplay for BufferAccessor<'a, R> {}
