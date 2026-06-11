//a Imports
use geo_nd::{matrix, quat, vector};

use crate::{Mat4, Quat, Vec3};

//a Transformation
//tp Transformation
/// A transformation corresponds to a translation of a rotation of a
/// scaling
///
/// The rotation here is encoded by a [Quat]ernion
#[derive(Clone, Copy, Debug)]
pub struct Transformation {
    /// Translation to apply after rotation
    translation: Vec3,
    /// Rotation to apply after scaling
    rotation: Quat,
    /// Scaling to apply first
    scale: Vec3,
}

impl std::default::Default for Transformation {
    fn default() -> Self {
        let translation = vector::zero();
        let scale = [1.; 3];
        let rotation = quat::new();
        Self {
            translation,
            scale,
            rotation,
        }
    }
}

//ip Transformation
impl Transformation {
    //fp new
    /// Create a new identity transformation
    pub fn new() -> Self {
        Default::default()
    }

    //cp with_scale
    /// Set the scaling of a transformation
    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    //cp with_translation
    /// Set the translation of a transformation
    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        self
    }

    //cp with_rotation
    /// Set the rotation of a transformation
    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self
    }

    //ap scale
    /// Get the scale
    pub fn scale(&mut self) -> Vec3 {
        self.scale
    }

    //ap translation
    /// Set the translation of a transformation
    pub fn translation(&mut self) -> Vec3 {
        self.translation
    }

    //ap rotation
    /// Set the rotation of a transformation
    pub fn rotation(&self) -> Quat {
        self.rotation
    }

    //mp set_scale
    /// Set the scaling of a transformation
    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    //mp set_translation
    /// Set the translation of a transformation
    pub fn set_translation(&mut self, translation: Vec3) {
        self.translation = translation;
    }

    //mp set_rotation
    /// Set the rotation of a transformation
    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    //cp copy_from
    /// Copy the transformation from another
    pub fn copy_from(&mut self, other: &Self) {
        self.translation = other.translation;
        self.scale = other.scale;
        self.rotation = other.rotation;
    }

    //mp combine
    /// Combine two transformations into this
    ///
    /// To operate correctly the scales must be
    pub fn combine(&mut self, base: &Self, other: &Self) {
        self.rotation = quat::multiply(&base.rotation, &other.rotation);
        self.translation = base.translation;
        self.translation = vector::add(self.translation, &other.translation, 1.);
        for i in 0..3 {
            self.scale[i] = base.scale[i] * other.scale[i];
        }
    }

    //mp translate
    /// Pre-apply a translation to the transformation
    pub fn translate(&mut self, translation: &Vec3, scale: f32) {
        self.translation = vector::add(self.translation, translation, scale);
    }

    //mp rotate_axis_angle
    /// Rotate the transformation by an angle about an axis
    pub fn rotate_axis_angle(&mut self, axis: &Vec3, angle: f32) {
        let q = quat::of_axis_angle(axis, angle);
        self.rotation = quat::multiply(&q, &self.rotation);
        // Glm.quat.multiply(self.translation, q, self.translation)
        // # self.translation = q * self.translation # type: ignore
    }

    //mp rotate_by
    /// Rotate the transformation by an angle about an axis
    pub fn rotate_by(&mut self, quaternion: &Quat) {
        self.rotation = quat::multiply(quaternion, &self.rotation);
    }

    //mp mat4
    /// Create a mat4 from the Transformation
    pub fn mat4(&self) -> Mat4 {
        let mut m = matrix::from_quat4(self.rotation);
        for i in 0..3 {
            m[4 * i] *= self.scale[i];
            m[4 * i + 1] *= self.scale[i];
            m[4 * i + 2] *= self.scale[i];
        }
        m[12] += self.translation[0];
        m[13] += self.translation[1];
        m[14] += self.translation[2];
        m
    }

    //mp mat4_inverse
    /// Create a mat4 from the inverse of this Transformation
    pub fn mat4_inverse(&self) -> Mat4 {
        let r = quat::conjugate(&self.rotation);
        let mut m = matrix::from_quat4(r);
        for i in 0..3 {
            let sc = 1. / self.scale[i];
            m[i] *= sc;
            m[i + 4] *= sc;
            m[i + 8] *= sc;
        }
        m[12] -= self.translation[0];
        m[13] -= self.translation[1];
        m[14] -= self.translation[2];
        m
    }

    //mp from_mat4
    /// Set this translation from a Mat4 (assuming it can be done)
    pub fn from_mat4(&mut self, m: Mat4) {
        self.translation = [m[12], m[13], m[14]];
        let mut rotation = [0.; 9];
        for i in 0..3 {
            let v = [m[4 * i], m[4 * i + 1], m[4 * i + 2]];
            let l = vector::length(&v);
            self.scale[i] = l;
            rotation[3 * i] = v[0] / l;
            rotation[3 * i + 1] = v[1] / l;
            rotation[3 * i + 2] = v[2] / l;
        }
        self.rotation = quat::of_rotation(&rotation);
    }

    //mp mat4_after
    /// Calculate a Mat4 of this transformation premultiplied by another Mat4
    pub fn mat4_after(&self, pre_mat: &Mat4) -> Mat4 {
        let m = self.mat4();
        matrix::multiply4(pre_mat, &m)
    }

    //mp interpolate
    /// Set this transformation to be an interpolation between two others
    pub fn interpolate(&mut self, t: f32, in0: &Self, in1: &Self) {
        let tn = 1.0 - t;
        for i in 0..3 {
            self.translation[i] = t * in0.translation[i] + tn * in1.translation[i];
            self.scale[i] = t * in0.scale[i] + tn * in1.scale[i];
        }
        self.rotation = quat::nlerp(t, &in0.rotation, &in1.rotation);
    }

    //mp distance
    /// Calculate an approximate 'distance' between two transformations
    pub fn distance(&self, other: &Self) -> f32 {
        let td = vector::distance(&self.translation, &other.translation);
        let sd = vector::distance(&self.scale, &other.scale);
        let qd = quat::distance(&self.rotation, &other.rotation);
        td + sd + qd
    }
    //zz All done
}

//ip Display for Transformation
impl std::fmt::Display for Transformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Transform +{:?}:@{:?}:*{:?}",
            self.translation, self.rotation, self.scale
        )
    }
}
