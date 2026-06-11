/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    object.rs
@brief   Part of 3D model library
 */

//a Imports
use crate::{TextureClient, VerticesClient, BufferClient};
use crate::{Transformation, Skeleton, Material, Component, Mesh, Vertices};
use crate::hierarchy;
use hierarchy::Hierarchy;

//a Object
//tp RenderableObject
/// Concept for a renderable object
pub struct RenderableObject<M, V>
where M:MaterialClient,
V:VerticesClient {
    /// Skeleton
    pub skeleton : Option<Skeleton>,
    /// All the vertices used
    pub vertices : Vec<V>,
    /// All the materials used
    pub materials: Vec<M>,
    /// The meshes etc that make up the object
    pub components   : Hierarchy<Component>,
}


