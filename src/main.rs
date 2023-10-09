use plotpy::Plot;
use tritet::{StrError, Tetgen};

fn main() -> Result<(), StrError> {
    // allocate data for 8 points
    let mut tetgen = Tetgen::new(8, None, None, None)?;

    // set points
    let marker = 0;
    tetgen
        .set_point(0, marker, 0.0, 0.0, 0.0)?
        .set_point(1, marker, 1.0, 0.0, 0.0)?
        .set_point(3, marker, 0.0, 1.0, 0.0)?
        .set_point(4, marker, 0.0, 0.0, 1.0)?
        .set_point(5, marker, 1.0, 0.0, 1.0)?
        .set_point(6, marker, 1.0, 1.0, 1.0)?
        .set_point(7, marker, 0.0, 1.0, 1.0)?;

    // generate Delaunay triangulation
    tetgen.generate_delaunay(false)?;

    // draw figure
    let mut plot = Plot::new();
    tetgen.draw_wireframe(
        &mut plot, true, // set_range
        true, // with_point_ids
        true, // with_triangle_ids
        true, // with_attribute_ids
        None, // fontsize_point_ids
        None, // fontsize_triangle_ids
        None, // fontsize_attribute_ids
    );

    // save figure
    plot.save("/tmp/tetgen-delaunay-1.svg")?;
    Ok(())
}
