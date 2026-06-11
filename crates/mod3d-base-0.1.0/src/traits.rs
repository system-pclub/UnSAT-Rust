use crate::{BufferAccessor, BufferData, Texture, VertexAttr, Vertices};
use crate::{MaterialAspect, MaterialBaseData, ShortIndex};

//a BufferClient
//tt BufferClient
/// Trait supported by a BufferData client
///
/// A buffer client is created first by a buffer as 'none'
///
/// The data may be created more than once with the same buffer; the client
/// is responsible for deduplication within the render context if required
pub trait BufferClient:
    Sized + std::fmt::Display + std::fmt::Debug + std::default::Default + Clone
{
}

//tt AccessorClient
/// Trait supported by a BufferAccessor client
///
/// A buffer client is created first by a buffer as 'none'
///
/// Before a view is creataed the data will be created at least once
///
/// The data may be created more than once with the same buffer; the client
/// is responsible for dedupliclation within the render context if required
pub trait AccessorClient:
    Sized + std::fmt::Display + std::fmt::Debug + std::default::Default + Clone
{
}

//tt TextureClient
/// The trait that must be supported by a client texture
///
/// Default is required as the client is made when a texture is made
/// Clone is required as the client is textures are cloned
pub trait TextureClient: Sized + std::fmt::Debug + std::default::Default + Clone {}

//tt MaterialClient
/// Trait supported by a material client
///
/// Default is not required as materials are only created in response
/// to a crate::Material
pub trait MaterialClient: Sized + std::fmt::Display + std::fmt::Debug {}

//tt VerticesClient
/// The trait that must be supported by a client vertices
///
/// Clone is required as Vertices can be borrowed by more than one object, and an
/// instantiable object contains the [VerticesClient] for the Vertices
///
pub trait VerticesClient: Sized + std::fmt::Debug + std::default::Default + Clone {}

//tt Renderable
/// The [Renderable] trait must be implemented by a type that is a
/// client of the 3D model system. It provides associated types for a
/// renderable context (this might be a particular shader program
/// within a OpenGL context, for example), and then its own structures
/// that are used to hold [BufferData], textures, materials, and sets
/// of renderable [Vertices].
pub trait Renderable: Sized {
    /// The renderer's type that reflects a [BufferData]
    type Buffer: BufferClient;
    /// The renderer's type that reflects a [BufferAccessor]
    type Accessor: AccessorClient;
    /// The renderer's type that represents a texture; this is
    /// supplied to material creation, and hence is less a product of
    /// the renderer and more an input to the 3D model library
    type Texture: TextureClient;
    /// The renderer's type that reflects a [Material]; this is expected
    /// to be an extraction of the aspects of a material that the
    /// renderer pipelines can apply.
    type Material: MaterialClient;
    /// The renderer's type that reflects a [BufferAccessor] of indices
    /// and the associated [BufferAccessor]s of attributes supported by a
    /// particular pipeline within the renderer
    type Vertices: VerticesClient;
    // type Instantiable : ;
    /// Initialize a buffer data client - it will have been created using default()
    fn init_buffer_data_client(
        &mut self,
        client: &mut Self::Buffer,
        buffer_data: &BufferData<Self>,
    );
    /// Initialize a buffer view client
    fn init_buffer_view_client(
        &mut self,
        client: &mut Self::Accessor,
        buffer_view: &BufferAccessor<Self>,
        attr: VertexAttr,
    );
    /// Create a client
    fn create_vertices_client(&mut self, vertices: &Vertices<Self>) -> Self::Vertices;
    /// Create a client
    fn create_texture_client(&mut self, texture: &Texture<Self>) -> Self::Texture;
    /// Create a client
    fn create_material_client<M>(
        &mut self,
        object: &crate::Object<M, Self>,
        material: &M,
    ) -> Self::Material
    where
        M: Material;

    /// Create a client for a reason - reason 0 is reserved
    /// Can we lose this?
    fn init_material_client<M: Material>(&mut self, client: &mut Self::Material, material: &M);
    // Destroy a client given a reason - reason 0 implies all
    // fn drop_material_client(&mut self, material: &dyn Material<Self>, render_context: &mut Self::Context);
}

//tt Material
/// A [Material] provides means to access the data for a material, be
/// it simple of full PBR. A fragment shader may require some aspects
/// of a material to be provided to it for rendering, and this API
/// allows that information to be gathered from any kind of material
pub trait Material: std::fmt::Debug {
    /// Invoked when an 3D model object is made renderable
    // fn create_renderable(&self, _render_context: &mut R::Context) {}

    /// Borrow the basic data of a material - color and base
    /// metallic/roughness, for example
    fn base_data(&self) -> &MaterialBaseData;
    /// Get the index into the Textures array for a specific aspect
    fn texture(&self, _aspect: MaterialAspect) -> ShortIndex {
        ShortIndex::none()
    }
}
