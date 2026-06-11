//a Imports
use std::rc::Rc;

use crate::{
    AccessorClient, BufferAccessor, BufferClient, BufferData, Material, MaterialClient, Renderable,
    Texture, TextureClient, VertexAttr, Vertices, VerticesClient,
};

//a Buffer
//tp Buffer
/// A Buffer, which is used for both a [BufferData] and a BufferAccessor client
///
/// This is a reference counted object - each [BufferData] has a
/// seperate one of these, and each [BufferAccessor] clones it so that if
/// there are N views then (after deconstruction of the object) a
/// Buffer will have a strong count of the number of views upon it
#[derive(Debug, Clone)]
pub struct Buffer(Rc<u32>);

//ip Display for Buffer
impl std::fmt::Display for Buffer {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

//ip Default for Buffer
impl Default for Buffer {
    fn default() -> Self {
        Self(Rc::new(0))
    }
}

//ip BufferClient for Buffer
impl BufferClient for Buffer {}

//ip AccessorClient for Buffer
impl AccessorClient for Buffer {}

//a Id
//tp Id
/// The thing that is Renderable - pretty much a place-holder
///
/// This is also used as a MaterialClient, TextureClient and VerticesClient
#[derive(Debug, Clone, Default)]
pub struct Id(u32);

//ip Display for Id
impl std::fmt::Display for Id {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

//ip MaterialClient for Id
impl MaterialClient for Id {}

//ip TextureClient for Id
impl TextureClient for Id {}

//ip VerticesClient for Id
impl VerticesClient for Id {}

//ip Renderable for Id
impl Renderable for Id {
    type Buffer = Buffer;
    type Accessor = Buffer;
    type Texture = Id;
    type Material = Id;
    type Vertices = Id;
    fn init_buffer_data_client(&mut self, _buffer: &mut Buffer, _data: &BufferData<Self>) {
        // No need to do anything; the
    }
    fn init_buffer_view_client(
        &mut self,
        client: &mut Self::Accessor,
        buffer_view: &BufferAccessor<Self>,
        _attr: VertexAttr,
    ) {
        buffer_view.data.create_client(self);
        *client = buffer_view.data.borrow_client().clone();
    }
    fn create_vertices_client(&mut self, _vertices: &Vertices<Self>) -> Self::Vertices {
        Self::Vertices::default()
    }
    fn create_texture_client(&mut self, _vertices: &Texture<Self>) -> Self::Texture {
        Self::Texture::default()
    }
    fn create_material_client<M>(
        &mut self,
        _object: &crate::Object<M, Self>,
        _material: &M,
    ) -> Self::Material
    where
        M: Material,
    {
        Self::Material::default()
    }
    fn init_material_client<M: Material>(&mut self, _client: &mut Self::Material, _material: &M) {}
}
