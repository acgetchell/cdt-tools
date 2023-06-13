extern crate blas;
extern crate lapack;
extern crate blas_src;

use plotpy::Plot;
use tritet::{StrError, Tetgen};

fn main() -> Result<(), StrError> {
    // allocate data for 8 points
    let mut tetgen = Tetgen::new(8, None, None, None)?;

    // set points
    tetgen
        .set_point(0, 0.0, 0.0, 0.0)?
        .set_point(1, 1.0, 0.0, 0.0)?
        .set_point(3, 0.0, 1.0, 0.0)?
        .set_point(4, 0.0, 0.0, 1.0)?
        .set_point(5, 1.0, 0.0, 1.0)?
        .set_point(6, 1.0, 1.0, 1.0)?
        .set_point(7, 0.0, 1.0, 1.0)?;

    // generate Delaunay triangulation
    tetgen.generate_delaunay(false)?;

    // draw edges of tetrahedra
    let _plot = Plot::new();

    Ok(())
}
