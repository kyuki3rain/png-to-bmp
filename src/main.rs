use std::{fs, io::Error, path::Path};

use clap::Parser;
use image::ImageFormat;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input_path: String,

    /// Number of times to greet
    #[arg(short, long)]
    output_path: String,
}

fn main() {
    let args = Args::parse();

    match convert_png_to_bmp(&args.input_path, &args.output_path) {
        Ok(_) => println!("変換が完了しました。"),
        Err(e) => println!("変換に失敗しました。: {}", e),
    }
}

fn convert_png_to_bmp(input_path: &str, output_path: &str) -> Result<(), image::ImageError> {
    let image = image::open(input_path)?; // PNG画像を開く

    create_directory(output_path)?;

    // BMP形式で保存する
    image.save_with_format(output_path, ImageFormat::Bmp)?;

    Ok(())
}

fn create_directory(path: &str) -> Result<(), Error> {
    let parent_dir = Path::new(path).parent().unwrap();
    fs::create_dir_all(parent_dir)
}
