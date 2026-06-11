//a Imports
use std::cell::RefCell;

use crate::{
    BufferAccessor, BufferData, BufferElementType, ByteBuffer, Renderable, ShortIndex, VertexAttr,
    Vertices,
};

//a ExampleBuffers
//tp ExampleBuffers
/// This is a monotonically increasing [Vec] of buffers, which are immutable once added to the struct
///
/// It allows the buffers to be borrowed (immutably) for the lifetime
/// of the structure, even if later more buffers are added to the Vec
///
/// Can remove the RefCell?
pub struct Buffers<'buffers> {
    buffers: RefCell<Vec<Box<dyn ByteBuffer + 'buffers>>>,
}

//ip Buffers
impl<'buffers> Buffers<'buffers> {
    //fp new
    /// Create a new empty [Buffers]
    pub fn new() -> Self {
        let buffers = Vec::new().into();
        Self { buffers }
    }

    //mp push
    /// Push a new [ByteBuffer] implementation and return its index
    pub fn push(&self, buffer: Box<dyn ByteBuffer>) -> usize {
        let mut buffers = self.buffers.borrow_mut();
        let n = buffers.len();
        buffers.push(buffer);
        n
    }

    //ap Borrow a buffer
    /// Create a new [BufferAccessor] on a particular [ByteBuffer] instance that has already been pushed
    pub fn buffer(&self, n: usize) -> &'buffers dyn ByteBuffer {
        let buffers = self.buffers.borrow();
        assert!(n < buffers.len(), "Buffer index out of range");
        let buffer = buffers[n].as_ref();
        unsafe { std::mem::transmute::<&'_ dyn ByteBuffer, &'buffers dyn ByteBuffer>(buffer) }
    }
}

//a DataAccessors
//tp DataAccessors
/// This structure helps for objects
pub struct DataAccessors<'buffers, R: Renderable> {
    data: Vec<Box<BufferData<'buffers, R>>>,
    accessors: Vec<Box<BufferAccessor<'buffers, R>>>,
}

//ip DataAccessors
impl<'buffers, R: Renderable> DataAccessors<'buffers, R> {
    //fp new
    /// Create a new [DataAccessors]
    pub fn new() -> Self {
        let data = Vec::new();
        let accessors = Vec::new();
        Self { data, accessors }
    }

    //fp push_buffer_data
    /// Push a new [BufferData] that is a portion of a Buffer
    pub fn push_buffer_data(
        &mut self,
        buffers: &Buffers<'buffers>,
        buffer_n: usize,
        byte_offset: u32,
        byte_length: u32,
    ) -> usize {
        let n = self.data.len();
        let b = buffers.buffer(buffer_n);
        let data = Box::new(BufferData::new(b, byte_offset, byte_length));
        self.data.push(data);
        n
    }

    //fp push_accessor
    /// Create a new [BufferAccessor] on a particular [BufferData] instance that has already been pushed
    pub fn push_accessor(
        &mut self,
        data: usize,
        num: u32,
        et: BufferElementType,
        ofs: u32,
        stride: u32,
    ) -> usize {
        let n = self.accessors.len();
        let d = unsafe {
            std::mem::transmute::<&BufferData<'_, R>, &'buffers BufferData<'buffers, R>>(
                &self.data[data],
            )
        };
        let accessor = Box::new(BufferAccessor::new(d, num, et, ofs, stride));
        self.accessors.push(accessor);
        n
    }

    //ap Accessor
    /// Create a new [BufferAccessor] on a particular [ByteBuffer] instance that has already been pushed
    pub fn accessor(&self, n: usize) -> &'buffers BufferAccessor<'buffers, R> {
        assert!(n < self.accessors.len(), "Accessor index out of range");
        let buffer = self.accessors[n].as_ref();
        unsafe {
            std::mem::transmute::<&BufferAccessor<'_, R>, &'buffers BufferAccessor<'buffers, R>>(
                buffer,
            )
        }
    }
}

//a ExampleVertices
//tp ExampleVertices
/// This structure provides for creating example objects, particularly with regard to their vertices
///
/// It uses arrays of [Pin]ned data structures so that the data can be safely self-referential
pub struct ExampleVertices<'buffers, R: Renderable> {
    buffers: Buffers<'buffers>,
    accessors: DataAccessors<'buffers, R>,
    vertices: Vec<Vertices<'buffers, R>>,
}

//ip Default for ExampleVertices
impl<'a, R: Renderable> Default for ExampleVertices<'a, R> {
    fn default() -> Self {
        Self::new()
    }
}

//ip ExampleVertices
impl<'a, R: Renderable> ExampleVertices<'a, R> {
    //fp new
    /// Create a new [ExampleVertices]
    ///
    /// This should probably not be Pin<Box<>>
    pub fn new() -> Self {
        let buffers = Buffers::new();
        let accessors = DataAccessors::new();
        let vertices = Vec::new();
        Self {
            buffers,
            accessors,
            vertices,
        }
    }

    //fp push_byte_buffer
    /// Push a new [ByteBuffer] implementation and return its index
    pub fn push_byte_buffer(&mut self, buffer: Box<dyn ByteBuffer>) -> usize {
        let buffer_n = self.buffers.push(buffer);
        self.accessors
            .push_buffer_data(&self.buffers, buffer_n, 0, 0)
    }

    //fp push_accessor
    /// Create a new [BufferAccessor] on a particular [ByteBuffer] instance that has already been pushed
    pub fn push_accessor(
        &mut self,
        data: usize,
        num: u32,
        et: BufferElementType,
        ofs: u32,
        stride: u32,
    ) -> usize {
        self.accessors.push_accessor(data, num, et, ofs, stride)
    }

    //fp push_vertices
    /// Create a new [Vertices] using a set of indices and positions
    ///
    /// This extends the life of the BufferAccessor to that of the ExampleVertices
    ///
    /// This is safe as the BufferAccessor's are in the Vec for ExampleVertices
    pub fn push_vertices(
        &mut self,
        indices: usize,
        positions: usize,
        attrs: &[(VertexAttr, usize)],
    ) -> ShortIndex {
        let n = self.vertices.len();
        let i = self.accessors.accessor(indices);
        let v = self.accessors.accessor(positions);
        let mut vertices = Vertices::new(i, v);
        for (attr, view_id) in attrs {
            let v = self.accessors.accessor(*view_id);
            vertices.add_attr(*attr, v);
        }
        self.vertices.push(vertices);
        n.into()
    }

    //fp borrow_vertices
    /// Borrow a set of vertices; this would allow (if mut!) the vertices to have attributes added
    pub fn borrow_vertices(&self, vertices: ShortIndex) -> &Vertices<R> {
        &self.vertices[vertices.as_usize()]
    }
}
