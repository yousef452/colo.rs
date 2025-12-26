mod colo;
use colo::*;

fn main() {
    let (width, height) = (31*20,31*20);
    let mut sys = System::init(width.try_into().unwrap(),height.try_into().unwrap(),Color::init_rgb(20,20,20));

    Text::init(
        "Hello".to_string(),
        "Minecraft.ttf".to_string(),
        30.0,
        30.0,
        Vector::init(width/2,height/2)
    ).unwrap().display(&mut sys);

    Circle::init(
        15*20,
        Vector::init(
            (width/2).try_into().unwrap(),
            (height/2).try_into().unwrap()
        ),
        Color::init_rgb(255,255,255)
    ).vertix(&mut sys);

    Rectangle::init(
        15*20,
        15*20,
        Vector::init(
            (width/2-15*20/2).try_into().unwrap(),
            (height/2-15*20/2).try_into().unwrap()
        ),
        Color::init_rgb(255,255,255)
    ).vertix(&mut sys);

    sys.image("gradient.png");
use std::fs;
}
