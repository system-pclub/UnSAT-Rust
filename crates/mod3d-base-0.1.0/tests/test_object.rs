use mod3d_base::example_client::Renderable;
use mod3d_base::Material;
use mod3d_base::{BufferAccessor, MaterialAspect};

// Create an object and interrogate it
// Create two distinct renderables
#[test]
fn test0() {
    // Create a triangle object with an empty skeleton
    let mut triangle = mod3d_base::ExampleVertices::new();
    mod3d_base::example_objects::triangle::new::<Renderable>(&mut triangle, 0.5);

    // Using the set of indices/vertex data defined create primitives (a triangle)
    let material = mod3d_base::BaseMaterial::of_rgba(0xff0000ff);
    let mut obj: mod3d_base::Object<mod3d_base::BaseMaterial, Renderable> =
        mod3d_base::Object::new();
    let v_id = obj.add_vertices(triangle.borrow_vertices(0.into()));
    let m_id = obj.add_material(&material);
    let mesh = mod3d_base::example_objects::triangle::mesh(v_id, m_id);
    obj.add_component(None, None, mesh);
    obj.analyze();
    let x = obj.vertices(v_id).borrow_client();
    let _p: &BufferAccessor<Renderable> = obj.vertices(v_id).borrow_indices();
    let _p = obj.material(m_id).texture(MaterialAspect::Normal);

    drop(x); // so we can desconstruct obj
    let inst = obj
        .into_instantiable(&mut Default::default())
        .map_err(|(_, e)| e)
        .expect("Failed to make the object instantiable");
    let r = &inst.render_recipe;
    assert_eq!(r.matrices.len(), 1, "Expected only an identity matrix");
    assert_eq!(
        r.matrices[0],
        [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.],
        "Expected only an identity matrix"
    );
    assert_eq!(r.primitives.len(), 1, "Expected only one primitive");
    assert_eq!(
        r.matrix_for_primitives.len(),
        1,
        "Expected only one primitive"
    );
    assert_eq!(
        r.matrix_for_primitives[0], 0,
        "Expected primitive to use identity"
    );
    // Want to interrogate obj?
    // Create a model3::renderable (given a 'shader')
    // Creating a renderable
}
