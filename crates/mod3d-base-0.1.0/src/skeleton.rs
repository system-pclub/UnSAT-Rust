//a Imports
use indent_display::{IndentedDisplay, IndentedOptions, Indenter, NullOptions};

use crate::hierarchy;
use crate::Bone;
use crate::Mat4;
use crate::Transformation;

//a Skeleton
//tp Skeleton
/// A set of related bones, with one or more roots
///
/// This corresponds to a skeleton (or a number thereof), with each
/// bone appearing once in each skeleton. The bones form a hierarchy.
#[derive(Debug)]
pub struct Skeleton {
    /// The bones that make up the set, with the hierarchical relationships
    pub skeleton: hierarchy::Hierarchy<Bone>,
    /// The roots of the bones and hierarchical recipes for traversal
    pub roots: Vec<(usize, hierarchy::Recipe)>,
    /// An array of matrices long enough for the one per level of traversal
    pub temp_mat4s: Vec<Mat4>,
    /// Max bone index
    pub max_index: usize,
}

//ip Default for Skeleton
impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

//ip Skeleton
impl Skeleton {
    //fp new
    /// Create a new set of bones
    pub fn new() -> Self {
        let skeleton = hierarchy::Hierarchy::new();
        let roots = Vec::new();
        let temp_mat4s = Vec::new();
        Self {
            skeleton,
            roots,
            temp_mat4s,
            max_index: 0,
        }
    }

    //mp add_bone
    /// Add a bone with a given base [Transformation] relative to its
    /// parent (if it has one), and an index to a Vec of Mat4 that the
    /// bone pose will utilize
    ///
    /// It returns the bone reference index
    pub fn add_bone(&mut self, transformation: Transformation, matrix_index: usize) -> usize {
        self.roots.clear();
        let bone = Bone::new(transformation, matrix_index);
        self.skeleton.add_node(bone)
    }

    //mp relate
    /// Relate a parent bone to a child bone (by bone reference indices)
    pub fn relate(&mut self, parent: usize, child: usize) {
        self.skeleton.relate(parent, child);
    }

    //mi find_max_matrix_index
    /// Find the maximum matrix index of all the bones (plus 1)
    fn find_max_matrix_index(&mut self) {
        let mut max_index = 0;
        for b in self.skeleton.borrow_elements() {
            if b.data.matrix_index >= max_index {
                max_index = b.data.matrix_index + 1
            }
        }
        self.max_index = max_index;
    }

    //mp resolve
    /// Resolve the [Skeleton] by finding the roots, generating
    /// traversal [hierarchy::Recipe]s for each root, allocating the
    /// required number of temporary [Mat4]s for the deepest of all
    /// the recipes, and finding the number of bone matrices required
    /// to be exported
    pub fn resolve(&mut self) {
        if self.roots.is_empty() {
            self.skeleton.find_roots();
            for r in self.skeleton.borrow_roots() {
                self.roots
                    .push((*r, hierarchy::Recipe::of_ops(self.skeleton.enum_from(*r))));
            }
            let mut max_depth = 0;
            for (_, recipe) in &self.roots {
                max_depth = if recipe.depth() > max_depth {
                    recipe.depth()
                } else {
                    max_depth
                };
            }
            self.temp_mat4s = Vec::new();
            for _ in 0..max_depth {
                self.temp_mat4s.push([0.; 16]);
            }
            self.find_max_matrix_index();
        }
    }

    //mp rewrite_indices
    /// Rewrite the bone matrix indices from 0 if required
    ///
    /// Each bone in the [Skeleton] is allocated the matrix index as it
    /// is reached through traversal from the roots of the [Skeleton].
    pub fn rewrite_indices(&mut self) {
        self.resolve();
        if self.max_index < self.skeleton.len() {
            let mut bone_count = 0;
            let (_, bones) = self.skeleton.borrow_mut();
            for (_, recipe) in &self.roots {
                for op in recipe.borrow_ops() {
                    if let hierarchy::NodeEnumOp::Push(n, _) = op {
                        bones[*n].data.matrix_index = bone_count;
                        bone_count += 1;
                    }
                }
            }
            self.max_index = bone_count;
        }
    }

    //mp derive_matrices
    /// Derive the matrices (as specified by [Bone]) for every bone in
    /// the [Skeleton] after the bones have been resolved.
    ///
    ///
    pub fn derive_matrices(&mut self) {
        assert!(
            !self.roots.is_empty(),
            "Resolve MUST have been invoked prior to derive_matrices"
        );
        let (_, bones) = self.skeleton.borrow_mut();
        let mut mat_depth = 0;
        for (_, recipe) in &self.roots {
            for op in recipe.borrow_ops() {
                match op {
                    hierarchy::NodeEnumOp::Push(n, _) => {
                        if mat_depth == 0 {
                            self.temp_mat4s[mat_depth] = *bones[*n]
                                .data
                                .derive_matrices(true, &self.temp_mat4s[mat_depth]);
                        } else {
                            self.temp_mat4s[mat_depth] = *bones[*n]
                                .data
                                .derive_matrices(false, &self.temp_mat4s[mat_depth - 1]);
                        }
                        mat_depth += 1;
                    }
                    _ => {
                        mat_depth -= 1;
                    }
                }
            }
        }
    }

    //fp iter_roots
    /// Iterate through the root bone indices in the [Skeleton]
    pub fn iter_roots(&self) -> impl Iterator<Item = usize> + '_ {
        self.roots.iter().map(|(n, _)| *n)
    }

    //zz All done
}

//ip IndentedDisplay for Skeleton
impl<'a, Opt: IndentedOptions<'a>> IndentedDisplay<'a, Opt> for Skeleton {
    //mp fmt
    /// Display for humans with indent
    fn indent(&self, f: &mut Indenter<'a, Opt>) -> std::fmt::Result {
        self.skeleton.indent(f)
    }
}

//ip Display for Skeleton
impl std::fmt::Display for Skeleton {
    //mp fmt
    /// Display for humans with indent
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut v = Vec::<u8>::new();
        let mut ind = Indenter::new(&mut v, " ", &NullOptions {});
        self.indent(&mut ind)?;
        drop(ind);
        write!(f, "{}", &String::from_utf8(v).unwrap())
    }
}
