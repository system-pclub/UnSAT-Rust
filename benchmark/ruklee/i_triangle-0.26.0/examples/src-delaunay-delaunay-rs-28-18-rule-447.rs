use i_triangle::delaunay::delaunay::Delaunay;
use i_triangle::delaunay::triangle::DTriangle;
use i_triangle::delaunay::vertex::DVertex;
use i_triangle::i_overlay::i_float::point::IntPoint;

fn main() {
    // Minimal Delaunay with one triangle whose vertex indices are far apart.
    // This makes `to_triangulation(shifted)` compute:
    //   *i_pnt.add(j) = a.index + shifted
    // with `j = 0, 1, 2`, but the backing `indices` vec has length only 3.
    //
    // The unsafe requirement for `ptr.add` is violated because the computed
    // offset is non-zero and the entire range between the base pointer and the
    // result is not within the allocated object.
    let tri = DTriangle::abc(
        0,
        DVertex::new(10_000, IntPoint::new(0, 0)),
        DVertex::new(20_000, IntPoint::new(1, 0)),
        DVertex::new(30_000, IntPoint::new(0, 1)),
    );

    let delaunay = Delaunay { triangles: vec![tri] };

    // Safe API only; this triggers out-of-bounds pointer arithmetic internally.
    let _ = delaunay.to_triangulation(0);

    println!("done");
}
