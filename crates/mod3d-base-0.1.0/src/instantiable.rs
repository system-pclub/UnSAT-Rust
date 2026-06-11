//a Imports
use crate::hierarchy;
use hierarchy::Hierarchy;

use crate::{Component, Instance, Material, RenderRecipe, Renderable, Skeleton, Texture, Vertices};

//a Instantiable
//tp Instantiable
/// An [Instantiable] is a type that is related to a set of Mesh data,
/// which can be instanced for different drawable::Instance's
///
/// It requires a related set of Mesh data that it does not refer to:
/// in object construction this Mesh data is likely to be the
/// structures containing vertex information and so on resident on a
/// CPU; in rendering the Mesh data is likely to be graphical objects
/// (such as OpenGL VAOs) that may reside (at least in part) on the
/// GPU.
///
/// The Instantiable data must be kept available to its related Instance.
///
/// The content of the Instantiable includes an array of Skeletons and
/// mesh transformation matrices, with appropriate index values. These
/// index values are into the related set of Mesh data.
#[derive(Debug)]
pub struct Instantiable<R>
where
    R: Renderable,
{
    /// Skeleton
    pub skeleton: Option<Skeleton>,
    /// All the vertices used
    pub vertices: Vec<R::Vertices>,
    /// All the textures used
    pub textures: Vec<R::Texture>,
    /// All the materials used
    pub materials: Vec<R::Material>,
    /// Render recipe
    pub render_recipe: RenderRecipe,
    /// Number of bone matrices required for all the bone sets in this structure
    pub num_bone_matrices: usize,
}

//ip Instantiable
impl<R> Instantiable<R>
where
    R: Renderable,
{
    //fp new
    /// Create a new instantiable drawable - something to which meshes
    /// and bone sets will be added, and for which a set of mesh
    /// matrices and rest bone positions will be derived.
    ///
    /// Such a type can that be 'instance'd with a specific
    /// transformation and bone poses, and such instances can then be
    /// drawn using shaders.
    pub fn new<M: Material>(
        skeleton: Option<Skeleton>,
        vertices: Vec<&Vertices<R>>,
        textures: Vec<&Texture<R>>,
        materials: Vec<R::Material>,
        mut components: Hierarchy<Component>,
    ) -> Self {
        components.find_roots();
        let render_recipe = RenderRecipe::from_component_hierarchy(&components);
        let num_bone_matrices = 0;
        let vertices = vertices
            .into_iter()
            .map(|v| v.borrow_client().clone())
            .collect();
        let textures = textures
            .into_iter()
            .map(|t| t.borrow_client().clone())
            .collect();
        Self {
            skeleton,
            vertices,
            textures,
            materials,
            render_recipe,
            num_bone_matrices,
        }
    }

    //mp instantiate
    /// Create an `Instance` from this instantiable - must be used with accompanying mesh data in the appropriate form for the client
    /// Must still add bone_poses one per bone set
    pub fn instantiate(&self) -> Instance<R> {
        Instance::new(self, self.num_bone_matrices)
    }

    //zz All done
}

/*
    //mp borrow_mesh_data
    /// Borrow the mesh data
    pub fn borrow_mesh_data (&self, index:usize) -> &MeshIndexData {
        &self.mesh_data[index]
    }

    pub fn add_meshes_of_node_iter(&self, meshes:&mut Vec<usize>, drawable:&mut drawable::Instantiable, iter:NodeIter<ObjectNode>) {
        let mut parent = None;
        let mut transformation = None;
        let mut bone_matrices = (0,0);
        let mut mesh_stack = Vec::new();
        for op in iter {
            match op {
                NodeIterOp::Push((n,obj_node), _has_children) => {
                    mesh_stack.push((parent, transformation, bone_matrices));
                    if let Some(bone_set) = obj_node.bones {
                        bone_matrices = drawable.add_bone_set(bone_set);
                    }
                    if let Some(obj_transformation) = &obj_node.transformation {
                        if transformation.is_none() {
                            transformation = Some(obj_transformation.mat4());
                        } else {
                            transformation = Some(matrix::multiply4(&transformation.unwrap(), &obj_transformation.mat4()));
                        }
                    }
                    if obj_node.mesh.is_some() {
                        let index = drawable.add_mesh(&parent, &transformation, &bone_matrices);
                        assert_eq!(index, meshes.len());
                        meshes.push(n);
                        parent = Some(index);
                        transformation = None;
                    }
                },
                NodeIterOp::Pop(_,_) => {
                    let ptb = mesh_stack.pop().unwrap();
                    parent = ptb.0;
                    transformation = ptb.1;
                    bone_matrices = ptb.2;
                },
            }
        }
    }

    pub fn create_instantiable(&mut self) -> drawable::Instantiable {
        self.nodes.find_roots();
        let mut drawable = drawable::Instantiable::new();
        let mut meshes = Vec::new();
        for r in self.nodes.borrow_roots() {
            self.add_meshes_of_node_iter(&mut meshes, &mut drawable, self.nodes.iter_from_root(*r));
        }
        self.meshes = meshes;
        drawable
    }
    pub fn bind_shader<'b, S:ShaderClass>(&self, drawable:&'b drawable::Instantiable, shader:&S) -> shader::Instantiable<'b> {
        let mut s = shader::Instantiable::new(drawable);
        for i in 0..self.meshes.len() {
            let obj_node = self.nodes.borrow_node(self.meshes[i]);
            assert!(obj_node.mesh.is_some(), "Mesh at node must be Some() for it to have been added to the self.meshes array by add_meshes_of_node_iter");
            let mesh = obj_node.mesh.unwrap();
            mesh.add_shader_drawables(shader, &mut s);
        }
        s
    }
*/
