//a Imports
use std::cell::{Ref, RefCell};

use crate::{BufferElementType, Renderable};

//a Texture
//tp Texture
/// A texture is managed by the library as a byte slice which has up
/// to three dimensions - minimally a width, with a 2D texture having
/// a non-zero height, and a 3D texture with a non-zero depth
///
/// The 'elements' that make up each entry of the texture can be
/// multiples of 1 to 4 of a fundamental element type (int or float,
/// of 8, 16 or 32 bits as permitted)
///
/// After the texture has been created, it may be instantiated within
/// the client, when a texture client handle is created by the client;
/// this must be easily Cloned, particuarly if the texture is used in
/// more than one instantiable object
pub struct Texture<'texture, R: Renderable + ?Sized> {
    /// The underlying data for the texture
    pub data: &'texture [u8],
    /// Width, height, and depth of the texture - width must be
    /// non-zero
    ///
    /// If height is zero then the texture is 1D, and depth must be 0
    ///
    /// If height is non-zero and depth is zero then the texture is 2D
    pub dims: (usize, usize, usize),
    /// Number of elements per texture entry (1,2,3 or 4)
    ///
    /// An RGB texture would be 3; an RGBA texture 4.
    pub elements_per_data: u32,
    /// The type of each element
    ///
    /// For most image textures this is Int8
    pub ele_type: BufferElementType,
    /// Client handle/value
    rc_client: RefCell<R::Texture>,
}

//ip Debug for Texture
impl<'texture, R: Renderable> std::fmt::Debug for Texture<'texture, R>
where
    R: Renderable,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(
            fmt,
            "Texture {{dims:{:?}, {:?}*{}, client:{:?}}}",
            self.dims, self.ele_type, self.elements_per_data, self.rc_client
        )?;
        Ok(())
    }
}

//ip Display for Texture
impl<'texture, R: Renderable> std::fmt::Display for Texture<'texture, R>
where
    R: Renderable,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "Texture:")?;
        writeln!(fmt, "  dims: {:?}", self.dims)?;
        Ok(())
    }
}

///ip Texture
impl<'texture, R: Renderable> Texture<'texture, R> {
    //cp new
    /// Create a new [Texture] object with no additional attributes
    pub fn new(
        data: &'texture [u8],
        dims: (usize, usize, usize),
        ele_type: BufferElementType,
        elements_per_data: u32,
    ) -> Self {
        let rc_client = Default::default();
        Self {
            data,
            dims,
            ele_type,
            elements_per_data,
            rc_client,
        }
    }

    //ap dims
    /// Get the dimensions of the texture
    pub fn dims(&self) -> &(usize, usize, usize) {
        &self.dims
    }

    //ap data
    /// Get the data slice for the texture
    pub fn data(&self) -> &[u8] {
        self.data
    }

    //ap data_type
    /// Get the data slice for the texture
    pub fn data_type(&self) -> (u32, BufferElementType) {
        (self.elements_per_data, self.ele_type)
    }

    //mp create_client
    /// Create the client texture
    pub fn create_client(&self, renderer: &mut R) {
        *(self.rc_client.borrow_mut()) = renderer.create_texture_client(self);
    }

    //ap borrow_client
    /// Borrow the client
    pub fn borrow_client(&self) -> Ref<R::Texture> {
        self.rc_client.borrow()
    }
}
