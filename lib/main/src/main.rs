use clap::Parser;
use image::{Rgb, RgbImage};
use seui_engine_raytracing_csg_renderer_core::{sample, types::rt::Scene};
use seui_engine_raytracing_csg_renderer_scene::DeserializableScene;
use seui_engine_raytracing_csg_renderer_types::{HDRColor, LDRColor};
use std::{fs::File, io::Read, path::Path};

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
    content: Vec<Vec<LDRColor>>,
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

fn load_scene(scene_file: &str, screen_aspect_ratio: f32) -> Scene {
    let mut file = File::open(scene_file).expect("Failed to open scene file");
    let mut content_str = String::new();
    file.read_to_string(&mut content_str)
        .expect("Failed to read scene file");

    serde_json::from_str::<DeserializableScene>(&content_str)
        .expect("Failed to parse scene JSON")
        .into_scene(screen_aspect_ratio)
}

fn tmp_hdr_to_ldr(color: HDRColor) -> LDRColor {
    const GAMMA: f32 = 2.2;
    const EXPOSURE: f32 = 1.0;

    let r = 1.0 - (-color.r * EXPOSURE).exp();
    let g = 1.0 - (-color.g * EXPOSURE).exp();
    let b = 1.0 - (-color.b * EXPOSURE).exp();

    LDRColor {
        r: r.powf(1.0 / GAMMA),
        g: g.powf(1.0 / GAMMA),
        b: b.powf(1.0 / GAMMA),
    }
}

fn main() {
    let args = Args::parse();

    let mut output_file = args.output.clone();
    if !args.no_output_png_suffix && !output_file.ends_with(".png") {
        output_file.push_str(".png");
    }

    let scene = load_scene(&args.scene, args.width as f32 / args.height as f32);

    let mut content = Vec::new();
    for y in 0..args.height {
        let mut row = Vec::new();
        for x in 0..args.width {
            row.push(tmp_hdr_to_ldr(sample(
                &scene,
                x as f32 / (args.width as f32 - 1.0),
                y as f32 / (args.height as f32 - 1.0),
            )))
        }
        content.push(row);
    }

    if let Err(e) = save_ldr_image(args.width, args.height, content, output_file) {
        eprintln!("Error saving image: {}", e);
        std::process::exit(1);
    }
}
