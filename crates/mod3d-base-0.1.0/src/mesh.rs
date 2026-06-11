//a Imports
use crate::Primitive;

//a Mesh
//tp Mesh
/// A [Mesh] provides an array of primitives, that is notionally drawn
/// from first to last
///
/// The [Mesh] depends on being in an 3D model object, as it is the
/// object that contains the actual materials and vertices to use
#[derive(Debug, Default)]
pub struct Mesh {
    /// The primitive
    pub primitives: Vec<Primitive>,
}

//ip Mesh
impl Mesh {
    //mp add_primitive
    /// Add a primitive to the [Mesh]
    pub fn add_primitive(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }

    //zz All done
}
