
// https://github.com/PistonDevelopers/image
extern crate image;
// TEST: filter: hue-rotate(180deg);
extern crate time;
//use time::PreciseTime;

use std::env;
use std::fs::File;
use std::path::Path;
use std::process::exit;

// [angle_deg] [in_image] [out_image]
fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() < 4 {
        println!("USAGE: hue-rotate [angle:int] [infile.img] [outfile.img]");
        exit(1);
    }
    let ref angle_deg = args[1];
    let ref in_image = args[2];
    let ref out_image = args[3];

    let angle = angle_deg.parse::<i32>().unwrap();
    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let img = image::open(&Path::new(in_image)).unwrap();
    // maybe add this to piston imageops/colorops methods


    // image::DynamicImage
    let imgbuf1 = img.hueroate(angle);// TODO img.huerotate(180)
    //let result1 = imgbuf1.save(&Path::new(out_image),image::ImageFormat::PNG);

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new(out_image)).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = imgbuf1.save(fout, image::PNG);

    exit(0);
    // https://github.com/PistonDevelopers/image/blob/0d3b652f6704886a287b3b9eb6e283809b438c04/src/imageops/sample.rs
    // https://github.com/PistonDevelopers/image/blob/0d3b652f6704886a287b3b9eb6e283809b438c04/src/imageops/mod.rs


}
