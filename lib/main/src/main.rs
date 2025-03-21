use clap::Parser;
use image::{Rgb, RgbImage};
use rayon::prelude::*;
use seui_engine_raytracing_csg_renderer_core::{sample, types::rt::Scene};
use seui_engine_raytracing_csg_renderer_long_double::LongDouble;
use seui_engine_raytracing_csg_renderer_scene::{DeserializableScene, Image, ImageLoader};
use seui_engine_raytracing_csg_renderer_types::{HDRColor, LDRColor};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::Arc,
};

/// Command-line arguments parser
#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    about = "Saves scene into image as PNG",
    rename_all = "kebab-case"
)]
struct Args {
    scene: String,
    output: String,
    #[arg(short, long)]
    no_output_png_suffix: bool,
    #[arg(short = 'W', long, default_value_t = 1920)]
    width: usize,
    #[arg(short = 'H', long, default_value_t = 1080)]
    height: usize,
    #[arg(long)]
    scene_type: Option<String>,
    #[arg(short = 'j', long, default_value_t = num_cpus::get())]
    threads: usize,
    #[arg(short, long, default_value_t = 1)]
    super_sampling: usize,
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
            let r = (pixel
                .r
                .clamp(LongDouble::from_f64(0.0), LongDouble::from_f64(1.0))
                * LongDouble::from_f64(255.0))
            .to_f64() as u8;
            let g = (pixel
                .g
                .clamp(LongDouble::from_f64(0.0), LongDouble::from_f64(1.0))
                * LongDouble::from_f64(255.0))
            .to_f64() as u8;
            let b = (pixel
                .b
                .clamp(LongDouble::from_f64(0.0), LongDouble::from_f64(1.0))
                * LongDouble::from_f64(255.0))
            .to_f64() as u8;
            img.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
        }
    }

    img.save(path)
}

enum SceneType {
    Jsonc,
    Yaml,
    Toml,
    Json5,
    Hjson,
}

fn load_scene(
    scene_file: &str,
    scene_type: &Option<String>,
    screen_aspect_ratio: LongDouble,
) -> Scene {
    let mut file = File::open(scene_file).expect("Failed to open scene file");
    let mut content_str = String::new();
    file.read_to_string(&mut content_str)
        .expect("Failed to read scene file");

    let image_loader = ImageImageLoader::new(scene_file);

    match match scene_type {
        None => {
            if scene_file.ends_with(".json") || scene_file.ends_with(".jsonc") {
                SceneType::Jsonc
            } else if scene_file.ends_with(".yaml") {
                SceneType::Yaml
            } else if scene_file.ends_with(".toml") {
                SceneType::Toml
            } else if scene_file.ends_with(".json5") {
                SceneType::Json5
            } else if scene_file.ends_with(".hjson") {
                SceneType::Hjson
            } else {
                panic!("Failed to recognize scene type")
            }
        }
        Some(t) if t == "json" || t == "jsonc" => SceneType::Jsonc,
        Some(t) if t == "yaml" => SceneType::Yaml,
        Some(t) if t == "toml" => SceneType::Toml,
        Some(t) if t == "json5" => SceneType::Json5,
        Some(t) if t == "hjson" => SceneType::Hjson,
        _ => {
            panic!("Unrecognized scene type")
        }
    } {
        SceneType::Jsonc => serde_jsonc2::from_str::<DeserializableScene>(&content_str)
            .expect("Failed to parse scene JSON")
            .into_scene(screen_aspect_ratio, &image_loader),
        SceneType::Yaml => serde_yaml::from_str::<DeserializableScene>(&content_str)
            .expect("Failed to parse scene YAML")
            .into_scene(screen_aspect_ratio, &image_loader),
        SceneType::Toml => toml::from_str::<DeserializableScene>(&content_str)
            .expect("Failed to parse scene TOML")
            .into_scene(screen_aspect_ratio, &image_loader),
        SceneType::Json5 => json5::from_str::<DeserializableScene>(&content_str)
            .expect("Failed to parse scene JSON5")
            .into_scene(screen_aspect_ratio, &image_loader),
        SceneType::Hjson => serde_hjson::from_str::<DeserializableScene>(&content_str)
            .expect("Failed to parse scene HJSON")
            .into_scene(screen_aspect_ratio, &image_loader),
    }
}

fn tmp_hdr_to_ldr(color: HDRColor) -> LDRColor {
    let gamma = LongDouble::from_f64(2.2);
    let exposure = LongDouble::from_f64(1.0);

    let r = LongDouble::from_f64(1.0) - (-color.r * exposure).exp();
    let g = LongDouble::from_f64(1.0) - (-color.g * exposure).exp();
    let b = LongDouble::from_f64(1.0) - (-color.b * exposure).exp();

    LDRColor {
        r: r.pow(LongDouble::from_f64(1.0) / gamma),
        g: g.pow(LongDouble::from_f64(1.0) / gamma),
        b: b.pow(LongDouble::from_f64(1.0) / gamma),
    }
}

fn main() {
    let args = Args::parse();
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .expect("Failed to set the number of threads");

    let mut output_file = args.output.clone();
    if !args.no_output_png_suffix && !output_file.ends_with(".png") {
        output_file.push_str(".png");
    }

    let scene = load_scene(
        &args.scene,
        &args.scene_type,
        LongDouble::from_f64(args.width as f64) / LongDouble::from_f64(args.height as f64),
    );

    let ss_factor = args.super_sampling;
    let inv_ss_factor =
        LongDouble::from_f64(1.0) / LongDouble::from_f64((ss_factor * ss_factor) as f64);

    let content: Vec<Vec<LDRColor>> = (0..args.height)
        .into_par_iter()
        .map(|y| {
            (0..args.width)
                .map(|x| {
                    let mut color = HDRColor {
                        r: LongDouble::from_f64(0.0),
                        g: LongDouble::from_f64(0.0),
                        b: LongDouble::from_f64(0.0),
                    };
                    for sy in 0..ss_factor {
                        for sx in 0..ss_factor {
                            let sample_x = (LongDouble::from_f64(x as f64)
                                + LongDouble::from_f64(sx as f64)
                                    / LongDouble::from_f64(ss_factor as f64))
                                / (LongDouble::from_f64(args.width as f64)
                                    - LongDouble::from_f64(1.0));
                            let sample_y = (LongDouble::from_f64(y as f64)
                                + LongDouble::from_f64(sy as f64)
                                    / LongDouble::from_f64(ss_factor as f64))
                                / (LongDouble::from_f64(args.height as f64)
                                    - LongDouble::from_f64(1.0));
                            color = color + sample(&scene, sample_x, sample_y);
                        }
                    }
                    tmp_hdr_to_ldr(color * inv_ss_factor)
                })
                .collect()
        })
        .collect();

    if let Err(e) = save_ldr_image(args.width, args.height, content, output_file) {
        eprintln!("Error saving image: {}", e);
        std::process::exit(1);
    }
}

struct ImageImage {
    image: RgbImage,
}

impl ImageImage {
    fn new(path: &str) -> ImageImage {
        let dyn_image = image::open(path).expect("Failed to load image");
        let rgb_image = dyn_image.to_rgb8();
        ImageImage { image: rgb_image }
    }
}

impl Image for ImageImage {
    fn width(&self) -> usize {
        self.image.width() as usize
    }

    fn height(&self) -> usize {
        self.image.height() as usize
    }

    fn get(&self, x: usize, y: usize) -> [LongDouble; 3] {
        if x >= self.width() || y >= self.height() {
            panic!("Incorrect coord given");
        }

        let pixel: &Rgb<u8> = self.image.get_pixel(x as u32, y as u32);

        [
            LongDouble::from_f64(pixel[0] as f64) / LongDouble::from_f64(255.0),
            LongDouble::from_f64(pixel[1] as f64) / LongDouble::from_f64(255.0),
            LongDouble::from_f64(pixel[2] as f64) / LongDouble::from_f64(255.0),
        ]
    }
}

struct ImageImageLoader {
    scene_dir: PathBuf,
}

impl ImageImageLoader {
    fn new<P: AsRef<Path>>(scene_path: P) -> Self {
        let scene_dir = scene_path
            .as_ref()
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf();
        ImageImageLoader { scene_dir }
    }
}

impl ImageLoader for ImageImageLoader {
    fn load(&self, path: &str) -> Arc<dyn Image + Send + Sync> {
        let full_path = self.scene_dir.join(path);
        Arc::new(ImageImage::new(full_path.to_str().expect("Invalid path")))
    }
}
