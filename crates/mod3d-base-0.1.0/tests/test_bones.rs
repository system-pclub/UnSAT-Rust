use mod3d_base::{Skeleton, Transformation};

fn build_bone_set() -> Skeleton {
    let mut skeleton = Skeleton::new();
    let b0 = skeleton.add_bone(Transformation::new(), 0);
    let b1 = skeleton.add_bone(Transformation::new().with_translation([1., 0., 0.]), 0);
    let b2 = skeleton.add_bone(Transformation::new().with_translation([0., 1., 0.]), 0);
    let b3 = skeleton.add_bone(Transformation::new().with_translation([0., 0., 1.]), 0);
    let b21 = skeleton.add_bone(Transformation::new().with_translation([0.5, 0., 0.]), 0);
    let b22 = skeleton.add_bone(Transformation::new().with_translation([0.0, 0., 0.5]), 0);
    skeleton.relate(b0, b1);
    skeleton.relate(b0, b2);
    skeleton.relate(b0, b3);
    skeleton.relate(b2, b21);
    skeleton.relate(b2, b22);
    skeleton.resolve();
    skeleton.rewrite_indices();
    skeleton
}

#[test]
fn test_0() {
    let skeleton = build_bone_set();
    println!("{}", skeleton);
    assert_eq!(1, skeleton.iter_roots().count());
    // assert!(false);
}

#[test]
fn test_1() {
    let mut skeleton = build_bone_set();
    skeleton.derive_matrices();
    println!("{}", skeleton);
    assert_eq!(
        skeleton.skeleton.borrow_node(4).borrow_mtb(),
        &[1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., -0.5, -1., 0., 1.]
    );
    assert_eq!(
        skeleton.skeleton.borrow_node(5).borrow_mtb(),
        &[1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., -1., -0.5, 1.]
    );
}
