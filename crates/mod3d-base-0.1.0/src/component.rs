//a Imports
use crate::{Mesh, Transformation};

//a Component
//tp Component
/// A [Component] of an object's hierarchy
///
/// Frequently an object will contain a single [Component] with no
/// transformation, just the mesh
#[derive(Debug)]
pub struct Component {
    /// The transformation to apply to the whole mesh
    pub transformation: Option<Transformation>,
    /// The mesh associated with the component
    pub mesh: Mesh,
}

//ip Component
impl Component {
    //fp new
    /// Create a new [Component]
    pub fn new(transformation: Option<Transformation>, mesh: Mesh) -> Self {
        Self {
            transformation,
            mesh,
        }
    }
}
