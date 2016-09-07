extern crate clap;
extern crate image;
extern crate rusty_machine as rm;

use clap::{Arg, App};
use image::RgbImage;
use rm::learning::k_means::KMeansClassifier;
use rm::prelude::*;

use std::usize;
use std::str::FromStr;
use std::path::{Path, PathBuf};

fn get_raw_image_mat(img: RgbImage) -> (u32, u32, Matrix<f64>) {
    // image meta data
    let width = img.width();
    let height = img.height();
    let pixel_count = (width * height) as usize;

    // Convert to a matrix of floats
    let data = img.into_raw();
    let float_data = data.iter().map(|&x| x as f64).collect::<Vec<_>>();
    (width, height, Matrix::new(pixel_count, 3, float_data))
}

fn create_compressed_image(width: u32,
                           height: u32,
                           classes: Vector<usize>,
                           original: &Matrix<f64>,
                           centroids: &Matrix<f64>)
                           -> RgbImage {
    let mut new_data = Vec::with_capacity(original.data().len());

    // Fill a `Vec` with the new pixel colours
    for output in classes.data().iter() {
        let colour = centroids.get_row(*output).expect("Output class not present in centroids");
        new_data.extend_from_slice(colour);
    }

    // Create from raw using the container
    RgbImage::from_raw(width,
                       height,
                       new_data.into_iter().map(|x| x as u8).collect::<Vec<_>>())
        .unwrap()
}

fn parse_cli() -> (usize, PathBuf, PathBuf) {
    let matches = App::new("Image Compression")
        .version("0.1.0")
        .author("James Lucas")
        .about("Uses rusty-machine's K-Means model to compress images")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .help("The input image path")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("The output image path")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("colors")
            .short("c")
            .value_name("INTEGER")
            .help("The number of colors present in the output image")
            .required(true)
            .takes_value(true))
        .get_matches();

    let colours_str = matches.value_of("colors").unwrap();
    let colours = usize::from_str(colours_str).expect("Color input must be an integer");

    let input_path = Path::new(matches.value_of("input").unwrap());
    assert!(input_path.exists(), "Input file does not exist");

    let output_path = Path::new(matches.value_of("output").unwrap());

    (colours, input_path.to_owned(), output_path.to_owned())
}   

fn main() {
    // Place-holders - we should parse input args
    let (colours, img_path, out_path) = parse_cli();

    // Open the image and convert it to a matrix with each row being a pixel
    println!("Loading image from {0}", img_path.display());
    let rgb_img = image::open(img_path).map(|img| img.to_rgb()).unwrap();
    let (width, height, rgb_mat) = get_raw_image_mat(rgb_img);

    println!("Matrix has {0} rows and {1} columns",
             rgb_mat.rows(),
             rgb_mat.cols());

    // The rusty-machine part!
    println!("Creating new K-Means model with {0} colours", colours);
    let mut model = KMeansClassifier::new(colours);
    println!("Training the model...");
    model.train(&rgb_mat);
    println!("Getting compressed colours from model...");
    let output_colors = model.predict(&rgb_mat);

    // Convert back to an RgbImage
    println!("Saving rgb image as {0}", out_path.display());
    let new_rgb_img = create_compressed_image(width,
                                              height,
                                              output_colors,
                                              &rgb_mat,
                                              model.centroids().as_ref().unwrap());
    new_rgb_img.save(out_path).expect("Could not save image");
}
