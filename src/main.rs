mod colo;
use colo::*;

fn main() {
    let (width, height) = (31*40,31*40);
    let mut sys = System::init(width.try_into().unwrap(),height.try_into().unwrap(),Color::init(0,0,0,0));
    println!("created the system");

    Rectangle::init(
        width*3/4,
        height*3/4,
        Vector::init(50,50),
        Color::init(0,255,0,175)
    ).display(&mut sys);
    println!("created the rectangle 0");

    Rectangle::init(
        width*3/4,
        height*3/4,
        Vector::init(150,150),
        Color::init(0,0,255,175)
    ).display(&mut sys);
    println!("created the rectangle 1");

    Rectangle::init(
        width*3/4,
        height*3/4,
        Vector::init(250,250),
        Color::init(255,0,0,175)
    ).display(&mut sys);
    println!("created the rectangle 2");

    Polygon::init(
        vec![
            Vector::init(width/2,0),
            Vector::init(width,height/2),
            Vector::init(width/2,height),
            Vector::init(0,height/2)
        ],
        Color::init_rgb(255,255,255)
    ).display(&mut sys);

    sys.ascii(40,"##".to_string());
    sys.image("gradient.png");
    println!("created the image");
}
