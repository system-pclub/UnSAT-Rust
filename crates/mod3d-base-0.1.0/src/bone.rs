//a Imports
use geo_nd::matrix;

use crate::{Mat4, Transformation};

//a Bone
//tp Bone
/// Each bone has a transformation with respect to its parent that is
/// a translation (its origin relative to its parent origin), scale
/// (in each direction, although a common scale for each coordinates
/// is best), and an orientation of its contents provided by a
/// quaternion (as a rotation).
///
/// A point in this bone's space is then translate(rotate(scale(pt)))
/// in its parent's space. The bone's children start with this
/// transformation too.
///
/// From this the bone has a local bone-to-parent transform matrix
/// and it has a local parent-to-bone transform matrix
///
/// At rest (where a mesh is skinned) there are two rest matrix variants
/// Hence bone_relative = ptb * parent_relative
///
/// The skinned mesh has points that are parent relative, so
/// animated_parent_relative(t) = btp(t) * ptb * parent_relative(skinned)
///
/// For a chain of bones Root -> A -> B -> C:
///  bone_relative = C.ptb * B.ptb * A.ptb * mesh
///  root = A.btp * B.btp * C.btp * C_bone_relative
///  animated(t) = A.btp(t) * B.btp(t) * C.btp(t) * C.ptb * B.ptb * A.ptb * mesh
#[derive(Debug)]
pub struct Bone {
    /// rest transform - translation, scale, rotation
    pub transformation: Transformation,
    /// The parent-to-bone mapping Matrix at rest; updated when the
    /// transformation is changed
    pub(crate) ptb: Mat4,
    /// The mesh-to-bone mapping Matrix at rest, derived from the
    /// hierarchy root; updated when any transformation is changed in
    /// the hierarchy at this bone or above
    pub(crate) mtb: Mat4,
    ///  Index into matrix array to put this bones animated mtm
    pub matrix_index: usize,
}

//ip Bone
impl Bone {
    //fp new
    /// Create a new bone with a given rest
    pub fn new(transformation: Transformation, matrix_index: usize) -> Self {
        let ptb = [0.; 16];
        let mtb = [0.; 16];
        Self {
            transformation,
            matrix_index,
            ptb,
            mtb,
        }
    }

    //mp borrow_transformation
    /// Borrow the transformation
    pub fn borrow_transformation(&self) -> &Transformation {
        &self.transformation
    }

    //mp set_transformation
    /// Set the transformation of the bone
    pub fn set_transformation(mut self, transformation: Transformation) -> Self {
        self.transformation = transformation;
        self
    }

    //mp derive_matrices
    /// Derive matrices for the bone given a parent mesh-to-bone [Mat4]
    pub fn derive_matrices(&mut self, is_root: bool, parent_mtb: &Mat4) -> &Mat4 {
        self.ptb = self.transformation.mat4_inverse();
        if is_root {
            self.mtb = self.ptb;
        } else {
            self.mtb = matrix::multiply4(&self.ptb, parent_mtb);
        }
        &self.mtb
    }

    //mp borrow_mtb
    /// Borrow the mtb Matrix (for test mainly)
    pub fn borrow_mtb(&self) -> &Mat4 {
        &self.mtb
    }

    //mp borrow_ptb
    /// Borrow the ptb Matrix (for test mainly)
    pub fn borrow_ptb(&self) -> &Mat4 {
        &self.ptb
    }
}

//ip Display for Bone
impl std::fmt::Display for Bone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Bone {} : {} : mtb={:?}",
            self.matrix_index, self.transformation, self.mtb
        )
    }
}

//ip DefaultIndentedDisplay for Bone
impl indent_display::DefaultIndentedDisplay for Bone {}
