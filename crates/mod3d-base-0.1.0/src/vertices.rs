//a Imports
use std::cell::{Ref, RefCell};

use crate::BufferAccessor;
use crate::{Renderable, VertexAttr};

//a Vertices
//tp Vertices
/// A set of vertices using one or more [crate::BufferData] through [BufferAccessor]s.
///
/// A number of [Vertices] is used by an `Object`, its components and their meshes; one is used for each primitive within a mesh for its elements.
/// The actual elements will be sets of triangles (as stripes or
/// whatever) which use these vertices.
///
/// A [Vertices] object includes a lot of options for vertices, and
/// different renderers (or different render stages) may require
/// different subsets of these indices. As such, in OpenGL for
/// example, a [Vertices] object may end up with more than one
/// `VAO`. This data is part of the [VerticesClient] struct
/// associated with the [Vertices]
///
/// When it comes to creating an instance of a mesh, that instance
/// will have specific transformations and materials for each of its
/// primitives; rendering the instance with a shader will require
/// enabling the [Vertices] client for that shader, setting
/// appropriate render options (uniforms in OpenGL)
#[derive(Debug)]
pub struct Vertices<'vertices, R: Renderable + ?Sized> {
    indices: &'vertices BufferAccessor<'vertices, R>,
    position: &'vertices BufferAccessor<'vertices, R>,
    rc_client: RefCell<R::Vertices>,
    attrs: Vec<(VertexAttr, &'vertices BufferAccessor<'vertices, R>)>,
}

//ip Display for Vertices
impl<'vertices, R: Renderable> std::fmt::Display for Vertices<'vertices, R>
where
    R: Renderable,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "Vertices:")?;
        writeln!(fmt, "  indices: {:?}", self.indices)?;
        writeln!(fmt, "  position: {:?}", self.position)?;
        for (n, a) in &self.attrs {
            writeln!(fmt, "  {n:?}: {a:?}")?;
        }
        Ok(())
    }
}

///ip Vertices
impl<'vertices, R: Renderable> Vertices<'vertices, R> {
    //fp new
    /// Create a new [Vertices] object with no additional attributes
    pub fn new(
        indices: &'vertices BufferAccessor<'vertices, R>,
        position: &'vertices BufferAccessor<'vertices, R>,
    ) -> Self {
        let attrs = Vec::new();
        let rc_client = RefCell::new(R::Vertices::default());
        Self {
            indices,
            position,
            rc_client,
            attrs,
        }
    }

    //mp add_attr
    /// Add a [BufferAccessor] for a particular [VertexAttr]
    ///
    /// On creation the [Vertices] will have views for indices and
    /// positions; this provides a means to add views for things such
    /// as normal, tex coords, etc
    pub fn add_attr(
        &mut self,
        attr: VertexAttr,
        accessor: &'vertices BufferAccessor<'vertices, R>,
    ) {
        self.attrs.push((attr, accessor));
    }

    //mp borrow_indices
    /// Borrow the indices [BufferAccessor]
    pub fn borrow_indices<'a>(&'a self) -> &'a BufferAccessor<'vertices, R> {
        self.indices
    }

    //mp borrow_position
    /// Borrow the position [BufferAccessor]
    pub fn borrow_position<'a>(&'a self) -> &'a BufferAccessor<'vertices, R> {
        self.position
    }

    //mp borrow_attr
    /// Borrow an attribute [BufferAccessor] if the [Vertices] has one
    pub fn borrow_attr<'a>(&'a self, attr: VertexAttr) -> Option<&'a BufferAccessor<'vertices, R>> {
        for i in 0..self.attrs.len() {
            if self.attrs[i].0 == attr {
                return Some(self.attrs[i].1);
            }
        }
        None
    }

    //mp iter_attrs
    /// Iterate through attributes
    pub fn iter_attrs(&self) -> std::slice::Iter<(VertexAttr, &BufferAccessor<'vertices, R>)> {
        self.attrs.iter()
    }

    //mp create_client
    /// Create the render buffer required by the BufferAccessor
    pub fn create_client(&self, renderer: &mut R) {
        self.indices.create_client(VertexAttr::Indices, renderer);
        self.position.create_client(VertexAttr::Position, renderer);
        for (attr, view) in self.iter_attrs() {
            view.create_client(*attr, renderer);
        }
        *(self.rc_client.borrow_mut()) = renderer.create_vertices_client(self);
    }

    //ap borrow_client
    /// Borrow the client
    pub fn borrow_client(&self) -> Ref<R::Vertices> {
        self.rc_client.borrow()
    }

    //zz All done
}
