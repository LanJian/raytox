pub mod algebra;

use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
//use algebra::Vector;

fn main() {
    let mut img = DynamicImage::new_rgb8(800, 600);
    let red = Rgba::from_channels(255, 0, 0, 255);
    for x in 0..img.width() {
        for y in 0..img.height() {
            img.put_pixel(x, y, red);
        }
    }
    img.save("out.png").unwrap();

}
