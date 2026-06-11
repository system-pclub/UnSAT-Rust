//a Documentation
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

/*!
# TODO

Replace ByteBuffer with T:AsRef<[u8]>

Make Vertices have an option<indices> and update renderable vertices clients

# 3D Model library

This library provides structures and functions to support simple and
complex 3D objects in a reasonably performant system. Its use cases
include 3D modeling tools, games, and 3D user interfaces.

The object model is derived from the Khronos glTF 3D
model/scene description (<https://github.com/KhronosGroup/glTF>),
without explicit support for animation or cameras.

## Overview of the model

The 3D model library is designed to provide for the description of 3D
objects with simple or sophisticated specifications of their vertices;
with materials that might be simple colors or complete PRBS
specifications, and so on.

Once models have been described they can be turned into GL-specific
instantiable structures. It is quite common for graphics libraries to
convert some external data form and to allocate internal memory for
rendering - so that the object description buffers used initially are
no longer required and their memory can be freed.

When an 'instantiable' object exists it can be rendered many times in
a single scene, with different transformations and skeleon poses.

Hence this library provides for [Instantiable] object creation that
first requires memory buffers to be allocated, descriptions of an
object created; from this [Object] the [Instantiable] is made for a
particular graphics library context - allowing the freeing of those
firstt memory buffers. This [Instantiable] can then be instanced a
number of times, and these [Instance] each have their own
[Transformation] and [SkeletonPose].

# Object creation

The first step in using the library is to create the required
[Object]. This requires some binary data buffer(s) containing float
vertex data (position, and possible normals, texture coordinates,
tangents, relevant bones and weights, and so on); also arrays of
unsigned int vertex indices, that indicate how the vertices form
strips of triangles, or lines, etc. In some applications this binary
data might come from a large file, containing the data for many
objecs; as such, different portions of the binary data contain
different parts of the object data.

The library requries that a data buffer support the [ByteBuffer]
trait, and then that data buffer can have [BufferData] views created
onto it - portions of the data buffer. For particular vertex data a
subset of [BufferData] is used to create a [BufferAccessor].

A number of [BufferData] are pulled together to produce a [Vertices]
object - this might describe all the points and drawing indices for a
complete object. Subsets of the [Vertices] object are combined with a
[Material] to help describe a set of elements to render - this might
be a TriangleStrip, for example - this is known as a [Primitive]

A [Component] of an object is a list of [Primitive] and a
[Transformation] (the list of [Primitive] is a [Mesh])

An [Object] has a hierarchy of [Component], and optionally a
[Skeleton]; it also must keep references to the data that it uses - so
it contains arrays of [Vertices] and [Material] references.

Note that the [Vertices] used by one object may be used by others; as
such one might load a single data file that contains many objects for
a game, and the objects all refer to the same buffers and even
[Material] and [Vertices].

## Buffers

Underlying the data model is the [ByteBuffer] trait - any data that is
used for the models must support this trait, and implementations are
provided for slice <> and for Vec<>.

### [BufferData]

A type that borrows a sub slice of[u8], using an explicit offset and
length, and which might have a client reference (e.g. an OpenGL
GlBuffer handle). It is similar to a Gltf BufferView, without a
'stride'.

The base concept for model [BufferData] is that it is an immutable
borrow of a portion of some model data buffer of a type that supports
the [ByteBuffer] trait; the data internally may be floats, ints, etc,
or combinations thereof - from which one creates [BufferAccessor]s, or
which it is itself used as model indices. So it can be the complete
data for a whole set of models.

Each [BufferData] has a related client element (a
[Renderable::Buffer]) which is created when an [Object] has its
client structures created; this may be an Rc of an OpenGL buffer, if
the client is an OpenGL renderer.

Each [BufferData] is use through one or more [BufferAccessor].

### {BufferAccessor]

A [BufferAccessor] is an immutable reference to a subset of a [BufferData]. A [BufferAccessor]
may, for example, be the vertex positions for one or more models; it may
be texture coordinates; and so on. The [BufferData] corresponds on the
OpenGL side to an ARRAY_BUFFER or an ELEMENT_ARRAY_BUFFER; hence it
expects to have a VBO associated with it.

The [BufferAccessor] is similar to a glTF Accessor.

Each [BufferAccessor] has a related client element (a
[Renderable::View]) which is created when an [Object] has its
client structures created; this may be the data indicating the subset
of the [Renderable::Buffer] that the view refers to, or perhaps a
client buffer of its own.

A set of [BufferAccessor]s are borrowed to describe [Vertices], each
[BufferAccessor] providing one piece of vertex information (such as
indices, position or normal). A single [BufferAccessor] may be used by
more than one [Vertices] object.

### [Vertices]

The [Vertices] type borrows at least one [BufferAccessor] for a vertex
indices buffer, and at least one [BufferAccessor] for positions of the
vertices; in addition it borrows more [BufferAccessor], one for each
attribute [VertexAttr] that is part of a mesh or set of meshes.

The [Vertices] object should be considered to be a complete descriptor
of a model or set of models within one or more [ByteBuffer]. In OpenGL
a Vertices object becomes a set of OpenGL Buffers (and subsets
thereof) and for a particular shader class it can be bound into a VAO.

A [Vertices] object is a set of related [BufferAccessor]s, with at least a
view for indices and a view for vertex positions; it may have more
views for additional attributes. It has a lifetime that is no longer
than that of the [BufferData] from which the [BufferAccessor]s are made.

A [Renderable::Vertices] can be constructed from a [Vertices]; this
is a renderer-specific vertices instance that replaces the use of
[BufferAccessor]s with the underlying client types.

## Skeleton and posing

A [Skeleton] consists of a (possibly multi-rooted) hierarchy of
[Bone]s. Each bone has a [Transformation], which is the mapping from
the coordinate space of its parent to the coordinate space of the bone
itself.

Each [Bone] has a 'matrix_index' which indicates which 'Joints' matrix
the bone is referred to by any 'joint's attribute entry in a mesh.

An object instance will have a [SkeletonPose] associated with it; this
allows the object contents to be rendered with adjustments to the
model, such as to make it appear to walk. A [SkeletonPose] is an array
of [BonePose] which reflect the associated [Bone]s in the skeleton;
each has an associated posed [Transformation].

The [SkeletonPose] can be traversed and for each posed bone an
appropriate mesh-to-model-space matrix can be generated; if a mesh is
annotated with bone weights that sum to 1 then a mesh vertex
coordinate can be converted to a posed-model coordinate by summing the
mesh-to-model-space matrices of the bones times their weights times
the mesh vertex coordinate.

A [Skeleton] is similar to a `skin` in GLTF.

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

## Textxures

A texture is a 1D, 2D or 3D object that is indexed by a shader to
provide one of a scalar, vec2, vec3 or vec4 of byte, short, integer,
or float.



## Materials

Materials are types that have the [Material] trait, and which have the
lifetime of the [BufferData] of the object they belong to; this is
because they may contain textures. As such they have an associated
Renderable::Material type, which has a lifetime as defined by the
Renderable.

[Material] is a trait that must be supported by materials, which thus
permits different abstract shading models to be used. It has a
`MaterialClient` parameter, which

Example [Material] instances are:

* [BaseMaterial] -

* [PbrMaterial] -

A [Material] has a make_renderable() method that makes it renderable?

## [Primitive]

A [Primitive] is a small structure that uses a single subset of
[Vertices] with a [Material] wih a drawing type (such as
TriangleStrip). It does not contain direct references to the vertices
or material; rather it uses an index *within* the arrays of [Vertices]
or [Material] held by an [Object].

A [Primitive] contains:

* a [Material] (from an index within the [Object] to which the primitive mesh belongs)

* a set of [Vertices] - the attributes required by the [Mesh] and a
  set of indices, a subset of which are used by the [Primitive] (from
  an index within the [Object] to which the mesh belongs)

* a drawable element type ([PrimitiveType] such as `TriangleStrip`)

* an index offset (within the [Vertices] indices)

* a number of indices

A [Primitive] does *not* contain any transformation information - all
the [Primitive] that belong to a [Mesh] have the same transformation.

## [Mesh]

A [Mesh] is an array of [Primitive]; this is just a way to combine sets
of drawn elements, all using the same transformation (other than bone
poses).

A mesh is part of a [Component] that is part of an [Object].

## [Component] of an Object

A [Component] is part of the hierarchy of an [Object] and has no
meaning without it; the indices and materials used in the [Component]
are provided by the [Object]. The [Component] has a [Transformation]
(relative to its parent) and a [Mesh].

Note that a hierarchy of object [Component]s is implicitly
`renderable` as it contains only indices, not actual references to
[BufferAccessor] data structures.

A hierarchy of object [Component]s can be reduced to a
[RenderRecipe]; this is an array of:

* transformation matrix

* material index (in a [Primitive])

* vertices index (in a [Primitive])

* drawable element type (in a [Primitive])

* index offset (in a [Primitive])

* index count (in a [Primitive])

## [Object]

An Object has an array of [Vertices] and [Materials] references that
its meshes use; both of these will end up being mapped to arrays of
graphic library client handles on a one-to-one basis (each [Vertices]
in the array becomes a graphics library vertices client, for example).

The Object then has a hierarchy of [Component]; this describes everything that it takes to draw the object.

Additionally the Object has an optional [Skeleton].

All of the data from an [Object] can be used to create an
[Instantiable]; this latter does not refer to any of he data buffers
(such as the [Vertices]) and so the original byte buffers can be
dropped once an [Instantiable] exists - all the data will be in the
graphics library.

## [Instantiable] objects

A 3D model [Object] consists of:

*  a hierarchy of [Component]s

*  a [Skeleton]

*  an array of [Vertices]; each of these is a set
of indices within a [BufferData] and attribute [BufferAccessor]s.

*  an array of [Material]

Such an object may have a plurality of render views created for it,
for use with different visualizers (in OpenGL these could be different
shaders, for example).

An object can be turned in to a renderable object within a
Renderable::Context using the `into_instantiable` method.  Once created
(unless the renderable context requires it) the object can be dropped.

The [Instantiable] is created within a specific renderable
context. For simple graphics libraries this probably means that
instances of the instantiated object are to be wih a single shader
program, whose attribute layout an uniforms is known ahead of time. As
such the renerable context might generate a single VAO for the
instantiable with appropriate buffer bindings, at the
into_instantiable invocation. For more complex applications, where an
object may be rendered with more than one layout of attributes, with
many different shader program classes, a VAO could be generated per
shader program class and (e.g.) a 'bind vertex buffers' for the
*object* can be invoked within a VAO prior to the rendering of a
number of the instances of the object with the shader program (each
presumably with its own 'uniform' settings!).

An [Instantiable] can then be drawn by
(theoretically, and given a particular [SkeletonPose]):

* Generating the [BonePose] mesh-to-model-space matrices for each bone in the [Skeleton]

* Traversing the hierarchy, keeping a current node [Transformation] in hand

* Apply the node's Transformation

* Render the node's [Primitive]s using the [Object]s material at the
  correct index, with the [Instantiable] associated with the
  [Vertices] index of the mesh

## Instantiated objects

An instantiated object is created by instantiating an [Instantiable].

The [Instance] has a [Transformation], a [SkeletonPose], and a set of
[Material] overrides; the [Material] overrides are an array of
optional materials.

For efficient rendering the object instance includes an array of the
instance's [SkeletonPose] matrices plus the base instance
[Transformation] matrix.

## Rendering an instance

A Vertices object is then used by a number of [Primitive]s; each of
these borrows the Vertices object, and it owns an array of
Drawables. Each Drawable is an OpenGL element type (such as
TriangleStrip), a number of indices, and an indication as to which
index within the Vertices object to use as the first index. Each Primitive has a single Material associated with it.

An array of Primitive objects is owned by a Mesh object, which corresponds to
the glTF Mesh - hence the Primitive object here corresponds to a glTF
Primitive within a glTF Mesh. A Mesh might correspond to a table leg or the table top in a model.

A number of Mesh objects are borrowed to form Object Nodes; each Node has its own Transformation that is applied to its Mesh. Hence a table can consist of four Object Nodes that borrow the same table leg Mesh, and a further Object Node that is the table top. A Node may also have a BoneSet associated with it, to provide for *skinned* objects.

An Object Node forms part of an Object's Hierarchy - hence the Object
Nodes are owned by the Object. An Object, then, can be a complete
table, for example. It might also be a posable robot; in this case one
would expect the top level node to have a BoneSet, and there to
perhaps be Object Nodes for the body, head, arms and legs, perhaps
sharing the same Mesh for the two arms and anoter Mesh for the two
legs.

It is worth noting at this point that the lifetime of an Object must
be no more than the lifetime of the data buffer containing its data;
even though the Object may be passed in to a GPU memory, the data used
for building the object then not being required by the CPU (using
STATIC_DRAW). It is, then, clearly useful to consider the Object as a
model *construction* type, not necessarily the model *draw* type.

When it comes to rendering an Object, this requires a Shader. The data
required by a Shader to render an Object depends not just on the
Object but also on the Shader's capabilities (such as does it utilize
vertex tangents). However, there is some data that is common for all
the Shaders that might render a single Object instance - such as the
bone poses for the object, and the mesh matrices.

Hence an Object must have two type created for it prior to rendering. The first is a drawable::Instantatiable. This is a drawable object that in itself may have instantiations.
The drawable::Instantiable contains an owned copy of the BoneSet for
the object, and any transformation data required by the meshes for
drawing (given each object node has its own transformation). The drawable::Instantiable is created from the object using its create_instantiable method.

The second type required for rendering is a shader::Instantiable. This
is used by binding the data required for a shader for an object to a
VAO (Vertex Attribute Object) for the shader; this VAO is used in the
rendering of any instances of the shader::Instantiable. The
shader::Instantiable borrows the drawable::Instantiable created for
the Object; it is created using the Object's bind_shader method.

With STATIC_DRAW the lifetime of the shader::Instantiable can be shorter than that of the Object data - it can have a lifetime of the rendering.

Once an Object has had all of its drawable::Instantiable and shader::Instantiable types created, the Object may be dropped.

A renderable instance of an object then requires a drawable::Instance
to be created from the drawable::Instantiable; this instance has its
own transformation and bone poses, and it borrows the
drawable::Instantiable. The Instance can be rendered with a particular
shader using that shader::Instantiable's `gl_draw` method, which takes
a reference to the Instance. This method then has access to all the
matrices required for the mesh, for the posed bones, and so on.

A Shader is created using standard OpenGL calls. It must have the ShaderClass trait.

An instantiable model consists of abstract mesh data and poses of the
elements within its skeleton, if required.

Multiple instances of an instantiable model can be created, each with its own set of poses.

The Hierarchy module provides for a hierarchy of owned elements which
are stored in an array inside the Hierachy structure; the
rerlationship between nodes in the hierarchy are handled by indices
into this array. The Hierarchy is designed to be created, and then
immutably interrogated - although the immutability refers to the
*hierarchy* and the *node array*, not the contents of the nodes - the
node content may be updated at will.

# Graphics libraries

In OpenGL we have

VAO = list of binding from VAO attr number to a buffer; it also has an
'elemeent array buffer' that is the buffer of indices. Note a buffer
here is an offset/stride of a gl buffer.

The VAO is specific to the shader class, as it uses the attribute locations of the shader

A VAO can have the attribute bbuffers bound iini one go with glBindVertexBuffers (or glVertexArrayVertexBuffers to specify the vao without binding it first),

The index buffer can be bound witth glBindElementBuffer.

In theory a single VAO can work for many objects with one shader class
as the attribute layour is specific o the shader class. This requires
all its buffers to be rebound and the element buffer to be rebound. So it keeps some of the VAO.

Uniforms must be set after useProgram

A VBO glBuffer can be a BufferData; an Accessor is the glBuffer with offset and stride.

## OpenGL 4.0

Program has id; attribute list of (AttribLocation, VertexAttr); uniform list of (UniformLocation, UniformId).

UniformId is either ViewMatrix, ModelMatrix, etc, User(x), or Buffer(x)

# Examples

use model3d::{BufferAccessor, MaterialAspect};
use model3d::example_client::Renderable;

# To do

Optimize primitive to fit within 32 bytes

Make Buffer have a client 'reproduce me' element so that if it comes
from a file that file could be reloaded if required. This would allow
the GPU data for an instantiable to be dropped and reloaded, if the
appropriate client code is written.  The Buffer would require this
element at creation time so that its create client method could could
capture it.

Add a String to each component, and extract that for each root component in the hierarchy
Maybe have an 'extract component by name' from object that creates an Instantiable (requires there to be no skeleton for now)

Make only part of an instantiable be drawn (have a Vec of RenderRecipes, one per component in the root by default)

!*/

mod types;
pub use types::BufferElementType;
pub use types::MaterialAspect;
pub use types::ShortIndex;
pub use types::{Mat3, Mat4, Quat, Vec3, Vec4};
pub use types::{PrimitiveType, VertexAttr};

//a To do
//
// Add index size to primitive (it is cache-line sensitive though)

//a Imports and exports
pub mod hierarchy;

mod transformation;
pub use transformation::Transformation;

mod bone;
mod bone_pose;
pub use bone::Bone;
pub use bone_pose::BonePose;

mod skeleton;
mod skeleton_pose;
pub use skeleton::Skeleton;
pub use skeleton_pose::SkeletonPose;

mod buffer_accessor;
mod buffer_data;
mod byte_buffer;
pub use buffer_accessor::BufferAccessor;
pub use buffer_data::BufferData;
pub use byte_buffer::ByteBuffer;

mod traits;
pub use traits::{
    AccessorClient, BufferClient, Material, MaterialClient, Renderable, TextureClient,
    VerticesClient,
};

mod texture;
pub use texture::Texture;

mod material;
pub use material::BaseData as MaterialBaseData;
pub use material::{BaseMaterial, PbrMaterial};

mod vertices;
pub use vertices::Vertices;
mod mesh;
mod primitive;
pub use mesh::Mesh;
pub use primitive::Primitive;

mod component;
pub use component::Component;
mod render_recipe;
pub use render_recipe::RenderRecipe;
mod object;
pub use object::Object;

mod instantiable;
pub use instantiable::Instantiable;
mod instance;
pub use instance::Instance;

pub mod example_objects;
pub use example_objects::ExampleVertices;

pub mod example_client;
