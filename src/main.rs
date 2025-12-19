mod boost;
use boost::*;

use image::{ImageBuffer, Rgb};

pub fn image() {
    let (width, height) = (31*20,31*20);
    let mut img = ImageBuffer::new(width, height);
    let mut sys = System::init(width.try_into().unwrap(),height.try_into().unwrap(),Color::init_rgb(20,20,20));

    let mut c = Circle::init(10*20,Vector::init(15*20,15*20),Color::init_rgb(20, 20, 20));

    let mut t = Tringle::init(Vector::init(3*20,2*20),Vector::init(12*20,7*20),Vector::init(6*20,14*20),Color::init_rgb(255, 255, 255));
    t.display(&mut sys);

    t = Tringle::init(Vector::init(20*20,7*20),Vector::init(26*20,1*20),Vector::init(25*20,14*20),Color::init_rgb(255, 255, 255));
    t.display(&mut sys);
    c.display(&mut sys);
    c = Circle::init(9*20,Vector::init(15*20,15*20),Color::init_rgb(255, 255, 255));
    c.display(&mut sys);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let (red, green, blue) = sys.get_pixel(x.try_into().unwrap(),y.try_into().unwrap()).unwrap().get_rgb();
        *pixel = Rgb([red, green, blue]);
    }
    img.save("gradient.png").unwrap();
}

fn main() {
    image();
}
