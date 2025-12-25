mod colo;
use colo::*;

use image::{ImageBuffer, Rgb};

fn main() {
    let (width, height) = (31*20,31*20);
    let mut img = ImageBuffer::new(width, height);
    let mut sys = System::init(width.try_into().unwrap(),height.try_into().unwrap(),Color::init_rgb(20,20,20));

    Polygon::init(
        vec![
            Vector::init(0,0),
            Vector::init(4*20,3*20),
            Vector::init(7*20,0),
            Vector::init(7*20,7*20),
            Vector::init(0,3*20)
        ],
        Color::init_rgb(255,255,255)
    ).display(&mut sys);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let (red, green, blue) = sys.get_pixel(x.try_into().unwrap(),y.try_into().unwrap()).unwrap().get_rgb();
        *pixel = Rgb([red, green, blue]);
    }
    img.save("gradient.png").unwrap();
}
