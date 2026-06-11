//a Imports
use crate::hierarchy;
use crate::Renderable;
use crate::{
    Component, Instantiable, Material, Mesh, ShortIndex, Skeleton, Texture, Transformation,
    Vertices,
};
use hierarchy::Hierarchy;

//a Object
//tp Object
/// A hierarchy of ObjectNode's
///
/// This can be flattened in to an Instantiable
pub struct Object<'object, M, R>
where
    R: Renderable,
    M: Material + 'object,
{
    /// Skeleton
    pub skeleton: Option<Skeleton>,
    /// All the vertices used
    pub vertices: Vec<&'object Vertices<'object, R>>,
    /// All the vertices used
    pub textures: Vec<&'object Texture<'object, R>>,
    /// All the materials used
    pub materials: Vec<&'object M>,
    /// The meshes etc that make up the object
    pub components: Hierarchy<Component>,
    // The roots of the bones and hierarchical recipes for traversal
    // pub roots   : Vec<(usize, Recipe)>,
    // Meshes - indices in to nodes.nodes array of the meshes in the order of instance
    // pub meshes : Vec<usize>
}

//ip Display for Object
impl<'object, M, R> std::fmt::Display for Object<'object, M, R>
where
    R: Renderable,
    M: Material + 'object,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "Object: {{")?;
        for v in &self.vertices {
            writeln!(fmt, "  {v}")?;
        }
        writeln!(fmt, "}}")
    }
}

//ip Default for Object
impl<'object, M, R> Default for Object<'object, M, R>
where
    M: Material + 'object,
    R: Renderable,
{
    fn default() -> Self {
        Self::new()
    }
}

//ip Object
impl<'object, M, R> Object<'object, M, R>
where
    M: Material + 'object,
    R: Renderable,
{
    //fp new
    /// Create a new [Object] with no components
    pub fn new() -> Self {
        let skeleton = None;
        let vertices = Vec::new();
        let textures = Vec::new();
        let materials = Vec::new();
        let components = Hierarchy::new();
        Self {
            skeleton,
            vertices,
            textures,
            materials,
            components,
        }
    }

    //ap vertices
    /// Borrow one of the vertices
    pub fn vertices(&self, n: ShortIndex) -> &Vertices<'object, R> {
        self.vertices[n.as_usize()]
    }

    //ap texture
    /// Borrow one of the textures
    pub fn texture(&self, n: ShortIndex) -> &Texture<'object, R> {
        self.textures[n.as_usize()]
    }

    //mp material
    /// Borrow a materiaal from the object
    pub fn material(&self, n: ShortIndex) -> &M {
        self.materials[n.as_usize()]
    }

    //mp add_vertices
    /// Add vertices to the object
    pub fn add_vertices(&mut self, vertices: &'object Vertices<'object, R>) -> ShortIndex {
        let n = self.vertices.len();
        self.vertices.push(vertices);
        n.into()
    }

    //mp add_texture
    /// Add texture to the object
    pub fn add_texture(&mut self, texture: &'object Texture<'object, R>) -> ShortIndex {
        let n = self.textures.len();
        self.textures.push(texture);
        n.into()
    }

    //fp add_material
    /// Add a material to the object
    pub fn add_material(&mut self, material: &'object M) -> ShortIndex {
        let n = self.materials.len();
        self.materials.push(material);
        n.into()
    }

    //fp add_component
    /// Add a component to the hierarchy
    pub fn add_component(
        &mut self,
        parent: Option<usize>,
        transformation: Option<Transformation>,
        mesh: Mesh,
    ) -> usize {
        let node = Component::new(transformation, mesh);
        let child = self.components.add_node(node);
        if let Some(parent) = parent {
            self.components.relate(parent, child);
        }
        child
    }

    //fp relate
    /// Add a relation between two components
    pub fn relate(&mut self, parent: usize, child: usize) {
        self.components.relate(parent, child);
    }

    //mp analyze
    /// Analyze the object once it has been completely created
    ///
    /// This must be performed before clients are created, render
    /// recipes are generated, or the object is deconnstructed into an
    /// [Instantiable].
    pub fn analyze(&mut self) {
        self.components.find_roots();
    }

    //dp into_instantiable
    /// Deconstruct the object into an [Instantiable] for the
    /// renderable. This should be invoked after analysis and clients
    /// have been created.
    ///
    /// This permits (for exampl in OpenGL) the CPU-side buffers in
    /// the object to be dropped, but the GPU-side objects (created by
    /// create_client) can be maintained. The [Instantiable] contains
    /// only instances of the types for the [Renderable].
    pub fn into_instantiable(self, renderer: &mut R) -> Result<Instantiable<R>, (Self, String)> {
        for v in &self.vertices {
            v.create_client(renderer);
        }
        for t in &self.textures {
            t.create_client(renderer);
        }
        let materials: Vec<R::Material> = self
            .materials
            .iter()
            .map(|m| renderer.create_material_client(&self, m))
            .collect();
        Ok(Instantiable::<R>::new::<M>(
            self.skeleton,
            self.vertices,
            self.textures,
            materials,
            self.components,
        ))
    }

    //zz All done
}
