// read first arg
// read roatation
// manipulate
// write file to target locations from third arg
use std::env;
// https://github.com/PistonDevelopers/image
extern crate image;

use image::GenericImage;
use std::fs::File;
use std::path::Path;


// [angle_deg] [in_image] [out_image]
fn main() {
    println!("Hello, world!");
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        println!("The first argument is {}. but need 3", args[1]);
        //exit 1;
    }
    let ref angle_deg = args[1]; //
    let ref in_image = args[2];
    let ref out_image = args[3];

    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let img = image::open(&Path::new(in_image)).unwrap();

    // The dimensions method returns the images width and height
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's ColorType
    println!("{:?}", img.color());

    let  (mut imgx, mut imgy) = (img.dimensions().0 as u32, img.dimensions().1 as u32);
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = img.get_pixel(x, y);
    }
    // Save the image as “fractal.png”
    // let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();
    //= note: expected type `&[u8]`
    //= note:    found type `image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>`
    //image::save_buffer(&Path::new("fractal.png"), imgbuf.pixels()., imgx, imgy, image::RGB(8)).unwrap();
    imgbuf.save(&Path::new(out_image));

    //let ref mut fout = File::create(&Path::new(out_image)).unwrap();
    // Write the contents of this image to the Writer in PNG format.
    //let _ = img.save(fout, image::PNG).unwrap();
}
