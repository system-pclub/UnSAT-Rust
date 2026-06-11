//a Imports
use crate::{PrimitiveType, ShortIndex};

//a Primitive
//tp Primitive
/// A primitive consisting of a material and a subset of
/// vertices using a particular range of indices
///
/// This might be, for example, the arm of a robot.
///
/// The [Primitive] depends on being in an 3D model [crate::Object] (or its derived [crate::Instantiable], as it is the
/// object that contains the actual materials and vertices to use
///
/// This *SHOULD* be optimized to fit within half a cache line (32 bytes)
///
/// Missing:
///
/// uses bones?
/// index type (u8, u16, u32) - is this part of indices?
#[derive(Debug, Clone)]
pub struct Primitive {
    /// Byte offset to first index to use
    ///
    /// If vertices_index is None then the first PrimitiveType to draw in the array
    byte_offset: u32,
    /// Number of indices to use
    index_count: u32,
    /// Material to be used in drawing - index within the [crate::Object]
    material_index: ShortIndex,
    /// Vertices index within the [crate::Object]
    ///
    /// This provides (effectively) the set of attribute `BufferView`s that the mesh utilizes
    ///
    /// May be 'None'
    vertices_index: ShortIndex,
    /// Type of the primitive (u8)
    primitive_type: PrimitiveType,
}

//ip Primitive
impl Primitive {
    //fp new
    /// Create a new Primitive from a Vertices
    ///
    /// use the indices' BufferView.ele_type: BufferElementType as index size
    pub fn new(
        primitive_type: PrimitiveType,
        vertices_index: ShortIndex,
        byte_offset: u32,
        index_count: u32,
        material_index: ShortIndex,
    ) -> Self {
        Self {
            byte_offset,
            index_count,
            material_index,
            vertices_index,
            primitive_type,
        }
    }

    //mp vertices
    /// Retrieve the data for the vertices in the primitive
    ///
    /// This is the vertices index, the offset index, and the count
    #[inline]
    pub fn vertices(&self) -> (Option<usize>, u32, u32) {
        (
            self.vertices_index.into(),
            self.byte_offset,
            self.index_count,
        )
    }

    //mp material
    /// Retrieve the material for the primitive - this is the material index
    #[inline]
    pub fn material(&self) -> ShortIndex {
        self.material_index as ShortIndex
    }

    //mp primitive_type
    /// Retrieve the [PrimitiveType] of the primitive
    #[inline]
    pub fn primitive_type(&self) -> PrimitiveType {
        self.primitive_type
    }

    //mp vertices_index
    /// Retrieve the index into the [crate::Object] vertices array that this
    /// primitive uses
    pub fn vertices_index(&self) -> ShortIndex {
        self.vertices_index
    }

    //mp material_index
    /// Retrieve the index into the [crate::Object] materials array that this
    /// primitive uses
    pub fn material_index(&self) -> ShortIndex {
        self.material_index
    }

    //mp index_count
    /// Get the number of indices required to draw this primitive
    pub fn index_count(&self) -> u32 {
        self.index_count
    }

    //mp byte_offset
    /// Get the byte offset within the indices buffer view to the
    /// first byte used by this primitive
    pub fn byte_offset(&self) -> u32 {
        self.byte_offset
    }

    //zz All done
}
