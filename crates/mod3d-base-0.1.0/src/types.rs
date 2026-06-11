//a Imports
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

//a Basic types
//tp Vec3
/// 3-dimensional vector
pub type Vec3 = [f32; 3];

//tp Vec4
/// 3-dimensional vector with extra coord (1 for position, 0 for direction)
pub type Vec4 = [f32; 4];

//tp Mat3
/// 3-by-3 matrix for transformation of Vec3
pub type Mat3 = [f32; 9];

//tp Mat4
/// 4-by-4 matrix for transformation of Vec4
pub type Mat4 = [f32; 16];

//tp Quat - Quaternion
/// Quaternion
pub type Quat = [f32; 4];

//a Buffer
//tp BufferElementType
/// The type of an element in a buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferElementType {
    /// 32-bit floating point
    Float32,
    /// 16-bit floating point
    Float16,
    /// 8-bit integers
    Int8,
    /// 16-bit integers
    Int16,
    /// 32-bit integers
    Int32,
}

//ip BufferElementType
impl BufferElementType {
    /// Get the length in bytes of the element type
    pub fn byte_length(self) -> usize {
        use BufferElementType::*;
        match self {
            Float32 => 4,
            Float16 => 2,
            Int8 => 1,
            Int16 => 2,
            Int32 => 4,
        }
    }
}

//a Drawing
/// A [VertexAttr] is a possible vertex attribute that can be used by
/// a renderer; a vertex always has a position attribute, but
/// additional attributes may or maynot be provided by a model
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VertexAttr {
    /// Indices
    Indices,
    /// Position (3xf32) of the point
    Position,
    /// Normal (3xf32) at the point
    Normal,
    /// Color at the point (4xf32)
    Color,
    /// Tangent at the point (4xf32?)
    Tangent,
    /// A set of joints (n x int)
    Joints,
    /// Weights (n x f16?) to apply to each bone\[joint\[i\]\]
    Weights,
    /// Texture coordinates (2 x f32)
    TexCoords0,
    /// Texture coordinates (2 x f32)
    TexCoords1,
    /// Texture coordinates (2 x f32)
    TexCoords2,
}

//tp PrimitiveType
/// Type of a primitive
///
/// This is set to match the GLTF
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PrimitiveType {
    /// Points (of an indeterminate size?)
    Points,
    /// Lines (n-1) (ab, cd, ef, ...)
    Lines,
    /// Close loop of (n) lines (ab, cd, ef, ..., za)
    LineLoop,
    /// Connected (n-1) lines (ab, bc, cd, de, ...)
    LineStrip,
    /// Individual (n/3) triangles (one for every three indices)
    #[default]
    Triangles,
    /// Strip of (n-2) triangles (abc, bcd, cde, def, ...)
    TriangleStrip,
    /// Fan of (n-2) triangles (abc, acd, ade, aef, ...)
    TriangleFan,
}

//tp MaterialAspect
/// The aspect of a material
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialAspect {
    /// Color (notionally RGBA as 4xf32)
    Color,
    /// Normal
    Normal,
    /// MetallicRoughness (notionally MR as 2xf32)
    MetallicRoughness,
    /// Occlusion (as f32)
    Occlusion,
    /// Emission (as f32)
    Emission,
}

//tp ShortIndex
/// An optional index used within the model system, that is up to 65000
///
/// It can be, effectively, 'None' or Some(usize less than 65000)
///
/// The purpose is to keep the size of indexed structures small and
/// permit the optional aspect; it is used to index Vec of textures,
/// vertices descriptor sets, etc
///
/// It has implementations of From<> to map a [usize] into a
/// [ShortIndex], and to map from [ShortIndex] to Option<usize>; plus
/// to map from Option<usize> (or anything that is Into<usize>) to a
/// ShortIndex, to ease use.
///
/// These extra implementations remove some of the type safety one
/// might have, but make it simpler to use the index
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortIndex(u16);

//ip Default for ShortIndex
impl std::default::Default for ShortIndex {
    fn default() -> Self {
        Self(65535)
    }
}

//ip ShortIndex
impl ShortIndex {
    ///cp none
    /// Create a 'None' value
    #[inline]
    pub fn none() -> Self {
        Default::default()
    }

    ///ap as_usize
    /// Return the value - if it is effectively None, then panic
    #[inline]
    pub fn as_usize(self) -> usize {
        assert!(self.0 != 65535);
        self.0 as usize
    }

    ///ap is_none
    /// Return true if the index is None
    #[inline]
    pub fn is_none(self) -> bool {
        self.0 == 65535
    }

    ///ap is_some
    /// Return true if the index is not None
    #[inline]
    pub fn is_some(self) -> bool {
        self.0 != 65535
    }
}

//ip From<usize> for ShortIndex
impl From<usize> for ShortIndex {
    fn from(index: usize) -> Self {
        assert!(index < 65535);
        Self(index as u16)
    }
}

//ip From<ShortIndex> for Option<usize>
impl From<ShortIndex> for Option<usize> {
    fn from(index: ShortIndex) -> Option<usize> {
        if index.is_none() {
            None
        } else {
            Some(index.as_usize())
        }
    }
}

//ip From<Option<into usize >> for ShortIndex
impl<I: Into<usize>> From<Option<I>> for ShortIndex {
    fn from(opt_index: Option<I>) -> Self {
        if let Some(index) = opt_index {
            let index: usize = index.into();
            assert!(index < 65535);
            Self(index as u16)
        } else {
            Self(65535_u16)
        }
    }
}
