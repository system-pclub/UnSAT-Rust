//a Imports
use std::cell::RefCell;

use crate::ByteBuffer;

//a Data
//tp Data
/// A data buffer for use with OpenGL vertex data. It may be indices
/// or vertex coordinates etc.
///
/// A data buffer may contain a lot of data per vertex, such as
/// position, normal, tangent, color etc.  a `View` on the data is
/// then a subset of this data - perhaps picking out just the
/// position, for example, for a set of vertices
///
/// The data buffer may, indeed, contain data for more than one object
/// - and the objects may have different data per vertex. The data
/// buffer is pretty free-form, it is a `View` on the `Data` which
/// identifies the object it applies to, and the vertex attributes
/// required
///
/// A data buffer may then be used by many `View`s. Each `View` may be
/// used by many primitives for a single model; alternatively,
/// primitives may have their own individual Views.
///
/// Of course the model may be instantiated many times in a single scene.
///
/// OpenGL will have one copy of the data for all the primitives and models.
pub struct Data<'a> {
    /// Data buffer itself
    data: &'a [u8],
    /// Offset in to the data buffer for the first byte
    byte_offset: usize,
    /// Length of data used in the buffer
    byte_length: usize,
    /// if a gl buffer then bound to data[byte_offset] .. + byte_length
    /// This will *either* be an ELEMENT_ARRAY_BUFFER or an ARRAY_BUFFER
    /// depending on how it is initially bound
    rc_gl_buffer: RefCell<gl::types::GLuint>,
}

//ip Data
impl<'a> Data<'a> {
    //fp new
    /// Create a new `Data` given a buffer, offset and length; if the
    /// length is zero then the whole of the data buffer post offset
    /// is used
    ///
    /// If offset and length are both zero, then all the data is used
    ///
    /// This function can be invoked prior to the OpenGL context being
    /// created; this performs no OpenGL calls
    pub fn new<B: ByteBuffer>(data: &'a B, byte_offset: usize, byte_length: usize) -> Self {
        let byte_length = {
            if byte_length == 0 {
                data.byte_length() - byte_offset
            } else {
                byte_length
            }
        };
        let rc_gl_buffer = RefCell::new(0);
        let data = data.borrow_bytes();
        Self {
            data,
            byte_offset,
            byte_length,
            rc_gl_buffer,
        }
    }

    //ap gl_buffer
    /// Get the gl_buffer associated with the data, assuming its
    /// `gl_create` method has been invoked at least once
    pub fn gl_buffer(&self) -> gl::types::GLuint {
        *self.rc_gl_buffer.borrow()
    }

    //mp gl_create_data
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    ///
    /// If this method is invoked more than once, only one OpenGL buffer is created
    pub fn gl_create_data(&self) {
        let gl_buffer = *self.rc_gl_buffer.borrow();
        if gl_buffer == 0 {
            unsafe {
                gl::GenBuffers(1, self.rc_gl_buffer.as_ptr());
                gl::BindBuffer(gl::ARRAY_BUFFER, *self.rc_gl_buffer.borrow());
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    self.byte_length as gl::types::GLsizeiptr,
                    self.data.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW,
                );
            }
        }
    }

    //mp gl_create_indices
    /// Create the OpenGL ELEMENT_ARRAY_BUFFER using STATIC_DRAW - this copies the data in to OpenGL
    ///
    /// If this method is invoked more than once, only one OpenGL buffer is created
    pub fn gl_create_indices(&self) {
        let gl_buffer = *self.rc_gl_buffer.borrow();
        if gl_buffer == 0 {
            unsafe {
                gl::GenBuffers(1, self.rc_gl_buffer.as_ptr());
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *self.rc_gl_buffer.borrow());
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    self.byte_length as gl::types::GLsizeiptr,
                    self.data.as_ptr() as *const gl::types::GLvoid,
                    gl::STATIC_DRAW,
                );
            }
        }
    }

    //mp gl_bind_indices
    /// Bind the data to the VAO ELEMENT_ARRAY_BUFFER as the indices buffer
    pub fn gl_bind_indices(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.gl_buffer());
        }
    }

    //zz All done
}

//ip Drop for Data
impl<'a> Drop for Data<'a> {
    //fp drop
    /// If an OpenGL buffer has been created for this then delete it
    fn drop(&mut self) {
        if self.gl_buffer() != 0 {
            unsafe {
                gl::DeleteBuffers(1, self.rc_gl_buffer.as_ptr());
            }
        }
    }
}

//ip Display for Data
impl<'a> std::fmt::Display for Data<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let data_ptr = self.data.as_ptr();
        write!(
            f,
            "Data[{:?}+{}#{}]:GL({})",
            data_ptr,
            self.byte_offset,
            self.byte_length,
            self.rc_gl_buffer.borrow()
        )
    }
}

//ip DefaultIndentedDisplay for Data
impl<'a> indent_display::DefaultIndentedDisplay for Data<'a> {}
