//a Documentation
/*!

This provides a function to create [ExampleVertices] object that is a triangle of a specified size at z=0

 */

//a Imports
use super::ExampleVertices;
use crate::{
    BufferElementType, Mesh, Primitive, PrimitiveType, Renderable, ShortIndex, VertexAttr,
};

/// Add new position, normal and indices views to an [ExampleVertices]
/// for a tetrahedron with base at z=0 and tip at (0, 0, size)
///
/// The bottom plane is reflectionallty symmetric about the X axis
///
/// This has four vertices with a normal at each that is directed away
/// from the centroid
///
/// The height is sqrt(2)/sqrt(3) * side length
/// The centroid is 1/4 of the way up = 1/sqrt(24)
/// sqrt(1-centroid^2) = sqrt(23/24)
///
/// Each equilateral triangle face has three sides of length size
///
/// The height of the triangles is size * sqrt(3)/2
/// The centroid of these triangles is at 1/3 of the height
///
/// The X tip (Y=0, Z=0) is then at 2/3 * sqrt(3)/2 * size = size / sqrt(3)
/// The other tips at Z=0 are then at
///  X = -1/3 * sqrt(3)/2 * size = -size / (2*sqrt(3))
///  Y = +- size/2
pub fn new<R: Renderable>(eg: &mut ExampleVertices<R>, size: f32) {
    let height = (2.0_f32 / 3.0).sqrt();
    let centroid = height / 4.0;
    let r3_2 = (3.0_f32).sqrt() * 0.5;
    let s = 1.0 / (3.0_f32).sqrt();
    let x = (23.0_f32 / 24.0).sqrt();
    let vertex_data = [
        size * 2.0 * s,
        0.,
        0.,
        x,
        0.,
        -centroid,
        -size * s,
        size * 0.5,
        0.,
        -x * 0.5,
        x * r3_2,
        -centroid,
        -size * s,
        -size * 0.5,
        0.,
        -x * 0.5,
        -x * r3_2,
        -centroid,
        0.,
        0.,
        size * height,
        0.,
        0.,
        1.,
    ];
    let index_data = [0u8, 1, 2, 3, 0, 1];

    let data_indices = eg.push_byte_buffer(Box::new(index_data));
    let data_vertices = eg.push_byte_buffer(Box::new(vertex_data));

    let indices = eg.push_accessor(data_indices, 6, BufferElementType::Int8, 0, 0);
    let normals = eg.push_accessor(data_vertices, 3, BufferElementType::Float32, 3 * 4, 6 * 4);
    let vertices = eg.push_accessor(data_vertices, 3, BufferElementType::Float32, 0, 6 * 4);

    // Create set of data (indices, vertex data) to by subset into by the meshes and their primitives
    eg.push_vertices(indices, vertices, &[(VertexAttr::Normal, normals)]);
}

/// Create a mesh for the tetrahedron given the vertices index and
/// material index within a parent model3d::Object
///
/// The object should have had the vertices for the tetrahedron (created
/// with new() above) added to it (using a parent [ExampleVertices])
pub fn mesh(v_id: ShortIndex, m_id: ShortIndex) -> Mesh {
    let mut mesh = Mesh::default();
    mesh.add_primitive(Primitive::new(
        PrimitiveType::TriangleStrip,
        v_id,
        0,
        6,
        m_id,
    ));
    mesh
}
