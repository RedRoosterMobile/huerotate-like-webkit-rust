// read first arg
// read roatation
// manipulate
// write file to target locations from third arg
use std::env;
use std::f64::consts::PI;
// https://github.com/PistonDevelopers/image
extern crate image;

use image::GenericImage;
// use std::fs::File;
use std::path::Path;
use std::process::exit;

fn clamp(num:f64) -> f64 {
    if (num < 0.0) {
        return 0.0;
    } else if (num > 255.0){
        return 255.0;
    } else {
        return num;
    }
}
// [angle_deg] [in_image] [out_image]
fn main() {
    println!("Hello, world!");
    let args: Vec<_> = env::args().collect();

    if args.len() < 4 {
        println!("The first argument is {}. but need 4", args[1]);
        exit(1);
    }
    let ref angle_deg = args[1]; //todo: convert to int and then to f64
    let ref in_image = args[2];
    let ref out_image = args[3];

    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let img = image::open(&Path::new(in_image)).unwrap();

    // The dimensions method returns the images width and height
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's ColorType
    println!("{:?}", img.color());

    let  (imgx, imgy) = (img.dimensions().0 as u32, img.dimensions().1 as u32);
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let mut matrix: [f64; 9] = [
        1.0, 0.0, 0.0,   // Reds
        0.0, 1.0, 0.0,   // Greens
        0.0, 0.0, 1.0    // Blues
    ];
    // dummy
    let angle = 3.0;
    let cosv = (angle * PI / 180.0).cos();
    let sinv = (angle * PI / 180.0).sin();
    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut original_pixel = img.get_pixel(x, y);
        // taken from webkit:
        // /Source/WebCore/platform/graphics/texmap/TextureMapperShaderProgram.cpp
        // todo: do manipulations
        matrix[0] = 0.213 + cosv * 0.787 - sinv * 0.213;
        matrix[1] = 0.715 - cosv * 0.715 - sinv * 0.715;
        matrix[2] = 0.072 - cosv * 0.072 + sinv * 0.928;

        matrix[3] = 0.213 - cosv * 0.213 + sinv * 0.143;
        matrix[4] = 0.715 + cosv * 0.285 + sinv * 0.140;
        matrix[5] = 0.072 - cosv * 0.072 - sinv * 0.283;

        matrix[6] = 0.213 - cosv * 0.213 - sinv * 0.787;
        matrix[7] = 0.715 - cosv * 0.715 + sinv * 0.715;
        matrix[8] = 0.072 + cosv * 0.928 + sinv * 0.072;


        *pixel = original_pixel;
    }

    imgbuf.save(&Path::new(out_image));
}
