//a Imports
use crate::hierarchy;
use hierarchy::{Hierarchy, NodeEnumOp};

use geo_nd::matrix;

use crate::Mat4;
use crate::{Component, Primitive};

//a RenderRecipe
//tp RenderRecipe
/// A [RenderRecipe] for a hierarchy of components
///
/// Created from a hierarchy of components, this is an array of
/// transformation matrices, an array of [Primitive]s, and an array of
/// pairs (matrix index, primitive index) of what needs to be drawn
#[derive(Debug)]
pub struct RenderRecipe {
    /// Matrices to use (the first is the identity matrix)
    pub matrices: Vec<Mat4>,
    /// The primitives within the component
    pub primitives: Vec<Primitive>,
    /// Draw requirements - matrix index for the associated primitive index
    pub matrix_for_primitives: Vec<usize>,
}

//ip Default RenderRecipe
impl Default for RenderRecipe {
    fn default() -> Self {
        Self::new()
    }
}

//ip RenderRecipe
impl RenderRecipe {
    //fp new
    /// Create a new [RenderRecipe]
    pub fn new() -> Self {
        let matrices = Vec::new();
        let primitives = Vec::new();
        let matrix_for_primitives = Vec::new();
        Self {
            matrices,
            primitives,
            matrix_for_primitives,
        }
    }

    //fp from_component_hierarchy
    /// Build a RenderRecipe from a [Hierarchy] of [Component]
    ///
    /// It requires the hierarchy to have had 'find_roots' executed prior
    pub fn from_component_hierarchy(components: &Hierarchy<Component>) -> Self {
        let mut recipe = Self::new();

        // Create matrices for all meshes in the component,
        // and enumerate them as (mesh index, matrix index) in `meshes`
        recipe.matrices.push(matrix::identity4());
        let mut meshes = Vec::new();
        for root in components.borrow_roots() {
            let mut trans_index = 0;
            let mut mesh_stack = Vec::new();
            for op in components.iter_from(*root) {
                match op {
                    NodeEnumOp::Push((n, comp), _has_children) => {
                        mesh_stack.push(trans_index);
                        if let Some(transformation) = comp.transformation {
                            let transformation = matrix::multiply4(
                                &recipe.matrices[trans_index],
                                &transformation.mat4(),
                            );
                            trans_index = recipe.matrices.len();
                            recipe.matrices.push(transformation);
                        } // else keep same trans_index as its parent
                        meshes.push((n, trans_index));
                    }
                    NodeEnumOp::Pop(_, _) => {
                        trans_index = mesh_stack.pop().unwrap();
                    }
                }
            }
        }

        // Copy out the mesh primitives paired with the matrix index
        for (n, trans_index) in meshes {
            for p in &components.borrow_node(n).mesh.primitives {
                recipe.primitives.push(p.clone());
                recipe.matrix_for_primitives.push(trans_index);
            }
        }

        recipe
    }
}
