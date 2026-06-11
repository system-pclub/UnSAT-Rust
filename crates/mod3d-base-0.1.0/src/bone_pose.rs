//a Imports
use geo_nd::matrix;

use crate::Bone;
use crate::{Mat4, Transformation};

//a BonePose
//tp BonePose
/// A pose of a [Bone], referring to it so that many poses can use the
/// same [Bone].
///
/// A [BonePose] has a current [Transformation] which indicates how it
/// is posed; the [Bone] contains mapping matrices for going from
/// bone-parent space to bone space, and for going from mesh space to
/// bone space
pub struct BonePose<'a> {
    /// The bone this corresponds to (within its hierarchy)
    bone: &'a Bone,
    /// transformation relative to bone rest
    transformation: Transformation,
    /// posed-bone-to-parent Mat4 derived from transformation
    pbtp: Mat4,
    /// Matrix mapping bone-space to mesh-space
    animated_btm: Mat4,
    /// Matrix mapping mesh-space to mesh-space
    animated_mtm: Mat4,
}

//ip BonePose
impl<'a> BonePose<'a> {
    //fp new
    /// Create a new pose of a bone
    pub fn new(bone: &'a Bone) -> Self {
        let transformation = *bone.borrow_transformation();
        let pbtp = [0.; 16];
        let animated_btm = [0.; 16];
        let animated_mtm = [0.; 16];
        Self {
            bone,
            transformation,
            pbtp,
            animated_btm,
            animated_mtm,
        }
    }

    //mp transformation_reset
    /// Reset the pose transformation to that of the bone in the skeleton
    pub fn transformation_reset(&mut self) {
        self.transformation = *self.bone.borrow_transformation();
    }

    //mp set_transformation
    /// Set a new pose transformation for the posed bone
    pub fn set_transformation(&mut self, transform: Transformation) {
        self.transformation = transform;
        self.pbtp = self.transformation.mat4();
    }

    //mp derive_animation
    /// Derive the animation matrices given a parent
    /// animated-posed-bone-to-mesh matrix
    ///
    /// If there is no parent (is_root true) then the animated
    /// bone-to-mesh is just the posed-bone-to-parent transformation
    ///
    /// If there is a parent then its pose transformation must be
    /// preapplied to this; when this animated_btm is applied to a
    /// vector (in this local bone space) one must first generate the vector
    /// in the parent-bone space (by applying pbtp) and then apply
    /// parent pbtm to generate model space
    ///
    /// Vectors in the model mesh space can be multiplied by the
    /// *bone*s mtb matrix to get a vector in this local bone space,
    /// to which the animated_btm can be applied to get a model space
    /// vector. Hence multiplying animated_btm and bone.mtb together.
    pub fn derive_animation(&mut self, is_root: bool, parent_animated_pbtm: &Mat4) -> &Mat4 {
        if is_root {
            self.animated_btm = self.pbtp;
        } else {
            self.animated_btm = matrix::multiply4(parent_animated_pbtm, &self.pbtp);
        }
        self.animated_mtm = matrix::multiply4(&self.animated_btm, &self.bone.mtb);
        &self.animated_btm
    }

    //mp borrow_animated_mtm
    /// Borrow the animated mesh-to-model-space matrix
    ///
    /// This assumes it has been derived
    #[inline]
    pub fn borrow_animated_mtm(&self) -> &Mat4 {
        &self.animated_mtm
    }

    //zz All done
}

//ip Display for BonePose
impl<'a> std::fmt::Display for BonePose<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Pose")?;
        write!(f, "  {}", self.bone)?;
        write!(f, "  {}", self.transformation)?;
        write!(f, "  {:?}", self.pbtp)?;
        write!(f, "  anim_btm: {:?}", self.animated_btm)?;
        write!(f, "  anim_mtm: {:?}", self.animated_mtm)?;
        Ok(())
    }
}

//ip DefaultIndentedDisplay for BonePose
impl<'a> indent_display::DefaultIndentedDisplay for BonePose<'a> {}
