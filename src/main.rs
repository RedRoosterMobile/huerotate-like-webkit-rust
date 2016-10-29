use std::env;
use std::f64::consts::PI;
// https://github.com/PistonDevelopers/image
extern crate image;

use image::GenericImage;
use std::path::Path;
use std::process::exit;

fn clamp_u8(num:f64) -> u8 {
    if num < 0.0 {
        return 0;
    } else if num > 255.0 {
        return 255;
    } else {
        return num as u8;
    }
}
// [angle_deg] [in_image] [out_image]
fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 4 {
        println!("USAGE: hue-rotate [angle:int] [infile.img] [outfile.img]");
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

    let angle = angle_deg.parse::<i32>().unwrap() as f64;

    let cosv = (angle * PI / 180.0).cos();
    let sinv = (angle * PI / 180.0).sin();
    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut original_pixel = img.get_pixel(x, y);
        // taken from webkit:
        // /Source/WebCore/platform/graphics/texmap/TextureMapperShaderProgram.cpp
        matrix[0] = 0.213 + cosv * 0.787 - sinv * 0.213;
        matrix[1] = 0.715 - cosv * 0.715 - sinv * 0.715;
        matrix[2] = 0.072 - cosv * 0.072 + sinv * 0.928;

        matrix[3] = 0.213 - cosv * 0.213 + sinv * 0.143;
        matrix[4] = 0.715 + cosv * 0.285 + sinv * 0.140;
        matrix[5] = 0.072 - cosv * 0.072 - sinv * 0.283;

        matrix[6] = 0.213 - cosv * 0.213 - sinv * 0.787;
        matrix[7] = 0.715 - cosv * 0.715 + sinv * 0.715;
        matrix[8] = 0.072 + cosv * 0.928 + sinv * 0.072;

        // array of u8 [u8; 4]
        let pixel_data = original_pixel.data;

        let r = pixel_data[0] as f64;
        let g = pixel_data[1] as f64;
        let b = pixel_data[2] as f64;

        original_pixel.data[0] = clamp_u8(matrix[0] * r + matrix[1] * g + matrix[2] * b);
        original_pixel.data[1] = clamp_u8(matrix[3] * r + matrix[4] * g + matrix[5] * b);
        original_pixel.data[2] = clamp_u8(matrix[6] * r + matrix[7] * g + matrix[8] * b);

        *pixel = original_pixel;
    }

    imgbuf.save(&Path::new(out_image));
}
