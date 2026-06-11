//a Imports
use crate::{Instantiable, Mat4, Renderable, SkeletonPose, Transformation};

//a Instance
//tp Instance
/// A drawable::Instance contains the instance data for an instance of
/// a drawable::Instantiable
///
/// It requires a base transformation, an array of BonePose (which
/// matches the Instantiable's BoneSet array), and an array of Mat4
/// for each bone in the BonePose array.
pub struct Instance<'a, R>
where
    R: Renderable,
{
    /// Reference to the [Instantiable] this is based on
    ///
    /// This is provided as the instance *depends* on the
    /// [Instantiable] although it does not use the data here
    ///
    /// The [Skeleton] of the [Instantiable] *is* borrowed by the
    /// [SkeletonPose]
    pub instantiable: &'a Instantiable<R>,
    /// The transformation to apply to this model instance
    pub transformation: Transformation,
    /// Matrix for the transformation (must be updated after updating Transformation),
    pub trans_mat: Mat4,
    /// The sets of BonePose corresponding to the BoneSet array in the Instantiable
    pub bone_poses: Vec<SkeletonPose<'a>>,
    /// Transformation matrices for the bones
    pub bone_matrices: Vec<Mat4>,
}

impl<'a, R> Instance<'a, R>
where
    R: Renderable,
{
    //fp new
    /// Create a new [Instance] from an [Instantiable]
    ///
    /// This contains an array of [SkeletonPose]s to allow elements of
    /// the [Instantiable] to be posed, and respective matrices for
    /// drawing the meshes within the [Instantiable]
    ///
    /// It should contain appropriate Materials too
    pub fn new(instantiable: &'a Instantiable<R>, num_bone_matrices: usize) -> Self {
        let transformation = Transformation::new();
        let trans_mat = [0.; 16];
        let bone_poses = Vec::new();
        let mut bone_matrices = Vec::with_capacity(num_bone_matrices);
        for _ in 0..num_bone_matrices {
            bone_matrices.push([0.; 16]);
        }
        Self {
            instantiable,
            transformation,
            trans_mat,
            bone_poses,
            bone_matrices,
        }
    }
}
