use clap::Parser;
use image::{Rgb, RgbImage};
use seui_engine_raytracing_csg_renderer_types::LDRPixel;
use std::path::Path;

/// Command-line arguments parser
#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Saves scene into image as PNG")]
struct Args {
    scene: String,

    output: String,

    #[arg(short, long)]
    no_output_png_suffix: bool,

    #[arg(short = 'W', long, default_value_t = 1920)]
    width: usize,

    #[arg(short = 'H', long, default_value_t = 1080)]
    height: usize,
}

pub fn save_ldr_image<P: AsRef<Path>>(
    width: usize,
    height: usize,
    content: Vec<Vec<LDRPixel>>,
    path: P,
) -> Result<(), image::ImageError> {
    let mut img = RgbImage::new(width as u32, height as u32);

    for (y, row) in content.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            let r = (pixel.r.clamp(0.0, 1.0) * 255.0) as u8;
            let g = (pixel.g.clamp(0.0, 1.0) * 255.0) as u8;
            let b = (pixel.b.clamp(0.0, 1.0) * 255.0) as u8;
            img.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
        }
    }

    img.save(path)
}

fn main() {
    let args = Args::parse();

    let mut output_file = args.output.clone();
    if !args.no_output_png_suffix && !output_file.ends_with(".png") {
        output_file.push_str(".png");
    }

    // No scene now

    let content = vec![
        vec![
            LDRPixel {
                r: 1.0,
                g: 0.0,
                b: 0.0
            };
            args.width
        ];
        args.height
    ];

    if let Err(e) = save_ldr_image(args.width, args.height, content, output_file) {
        eprintln!("Error saving image: {}", e);
        std::process::exit(1);
    }
}
