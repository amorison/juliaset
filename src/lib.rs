use ndarray::Array2;
use num_complex::Complex64;

pub struct ComplexRegion {
    xleft: f64,
    xright: f64,
    yleft: f64,
    yright: f64,
}

impl ComplexRegion {
    pub fn new(xleft: f64, xright: f64, yleft: f64, yright: f64) -> Self {
        assert_ne!(xleft, xright);
        assert_ne!(yleft, yright);
        Self {
            xleft,
            xright,
            yleft,
            yright,
        }
    }

    pub fn xspan(&self) -> f64 {
        (self.xright - self.xleft).abs()
    }

    pub fn yspan(&self) -> f64 {
        (self.yright - self.yleft).abs()
    }

    pub fn build(&self, resolution_x: usize) -> Array2<Complex64> {
        let asp = self.yspan() / self.xspan();
        let nx: f64 = resolution_x as f64;
        let ny = (nx * asp).round();
        let dx = self.xright - self.xleft;
        let dy = self.yright - self.yleft;
        Array2::from_shape_fn((resolution_x, ny as usize), |(i, j)| {
            let x = i as f64 / nx * dx + self.xleft;
            let y = j as f64 / ny * dy + self.yleft;
            Complex64::new(x, y)
        })
    }
}

pub struct JuliaDiv {
    c_0: Complex64,
    threshold: f64,
    n_iterations: (usize, usize),
    resolution: usize,
}

impl JuliaDiv {
    pub fn new(
        c_0: Complex64,
        threshold: f64,
        n_iterations: (usize, usize),
        resolution: usize,
    ) -> Self {
        assert!(n_iterations.0 < n_iterations.1);
        Self {
            c_0,
            threshold,
            n_iterations,
            resolution,
        }
    }

    pub fn over(&self, plane: ComplexRegion) -> Array2<f64> {
        let (itermin, itermax) = self.n_iterations;
        let thres_sqr = self.threshold.powi(2);
        plane.build(self.resolution).mapv_into_any(|mut z_s| {
            let mut i = 0;
            loop {
                z_s = z_s * z_s + self.c_0;
                if i == itermax || z_s.norm_sqr() > thres_sqr {
                    break;
                }
                i += 1;
            }
            (i.max(itermin) - itermin) as f64 / (itermax - itermin) as f64
        })
    }
}
