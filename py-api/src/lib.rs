use numpy::{Complex64, PyArray2};
use pyo3::prelude::*;

/// Define an area of the complex plane.
#[pyclass(frozen)]
pub struct ComplexRegion(::juliaset::ComplexRegion);

#[pymethods]
impl ComplexRegion {
    #[new]
    fn new(xleft: f64, xright: f64, yleft: f64, yright: f64) -> Self {
        Self(::juliaset::ComplexRegion::new(xleft, xright, yleft, yright))
    }

    /// Create an array spanning the region with the given resolution along the real axis.
    fn build<'py>(&self, py: Python<'py>, resolution: usize) -> &'py PyArray2<Complex64> {
        let out = self.0.build(resolution);
        PyArray2::from_owned_array(py, out)
    }
}

/// Calculator of a Julia set
#[pyclass(frozen)]
pub struct JuliaDiv(::juliaset::JuliaDiv);

#[pymethods]
impl JuliaDiv {
    #[new]
    #[args(threshold = "2.0", n_iterations = "(0, 50)", resolution = "1000")]
    fn new(
        c_0: Complex64,
        threshold: f64,
        n_iterations: (usize, usize),
        resolution: usize,
    ) -> Self {
        Self(::juliaset::JuliaDiv::new(
            c_0,
            threshold,
            n_iterations,
            resolution,
        ))
    }

    /// Compute the divergence map on the desired region
    fn over<'py>(&self, py: Python<'py>, region: &ComplexRegion) -> &'py PyArray2<f64> {
        let out = self.0.over(&region.0);
        PyArray2::from_owned_array(py, out)
    }
}

/// Routines to compute Julia sets for imaging purposes
#[pymodule]
fn juliaset(_py: Python<'_>, pymod: &PyModule) -> PyResult<()> {
    pymod.add_class::<ComplexRegion>()?;
    pymod.add_class::<JuliaDiv>()?;
    Ok(())
}
