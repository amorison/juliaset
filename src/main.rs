use clap::Parser;
use juliaset::{ComplexRegion, JuliaDiv};
use num_complex::Complex64;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of pixels in real (x) direction
    #[clap(short = 'R', long, value_parser, default_value_t = 500)]
    resolution: usize,

    /// Real (x) value on left side of image
    #[clap(short, long, value_parser, default_value_t = -1.6)]
    xleft: f64,

    /// Real (x) value on right side of image
    #[clap(short = 'X', long, value_parser, default_value_t = 1.6)]
    xright: f64,

    /// Imaginary (y) value on bottom side of image
    #[clap(short, long, value_parser, default_value_t = -1.0)]
    ybot: f64,

    /// Imaginary (y) value on top side of image
    #[clap(short = 'Y', long, value_parser, default_value_t = 1.0)]
    ytop: f64,

    /// Real part of constant
    #[clap(short = 'r', long, value_parser, default_value_t = -0.835)]
    real: f64,

    /// Imaginary part of constant
    #[clap(short = 'i', long, value_parser, default_value_t = -0.2321)]
    imag: f64,

    /// Threshold for divergence
    #[clap(short = 't', long, value_parser, default_value_t = 2.0)]
    threshold: f64,

    /// Minimum number of iterations for coloring
    #[clap(short = 'm', long, value_parser, default_value_t = 5)]
    itermin: usize,

    /// Maximum number of iterations
    #[clap(short = 'M', long, value_parser, default_value_t = 80)]
    itermax: usize,
}

fn main() {
    let args = Args::parse();

    let area = ComplexRegion::new(args.xleft, args.xright, args.ytop, args.ybot);
    let julia_div = JuliaDiv::new(
        Complex64::new(args.real, args.imag),
        args.threshold,
        (args.itermin, args.itermax),
        args.resolution,
    );
    let img = julia_div.over(area);
    let img = img.t().map(|lum| (lum * 255.0) as u8);
    image::save_buffer(
        "plot.png",
        img.as_standard_layout().as_slice().unwrap(),
        img.shape()[1]
            .try_into()
            .expect("image width doesn't fit in u32"),
        img.shape()[0]
            .try_into()
            .expect("image height doesn't fit in u32"),
        image::ColorType::L8,
    )
    .expect("error when saving the image");
}
