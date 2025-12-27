mod colo;
use colo::*;



fn main() {
    let (width, height) = (31*200,31*200);
    let mut sys = System::init_white(width.try_into().unwrap(),height.try_into().unwrap());
    println!("created the system");

    Rectangle::init(
        29*200,
        29*200,
        Vector::init(
            (width/2-29*200/2).try_into().unwrap(),
            (height/2-29*200/2).try_into().unwrap()
        ),
        Color::init_rgb(0,0,0)
    ).display(&mut sys);
    println!("created rectangle 1");

    Rectangle::init(
        28*200,
        28*200,
        Vector::init(
            (width/2-28*200/2).try_into().unwrap(),
            (height/2-28*200/2).try_into().unwrap()
        ),
        Color::init_rgb(183, 65, 14)
    ).display(&mut sys);
    println!("created rectangle 2");

    Rectangle::init(
        27*200,
        27*200,
        Vector::init(
            (width/2-27*200/2).try_into().unwrap(),
            (height/2-27*200/2).try_into().unwrap()
        ),
        Color::init_rgb(0,0,0)
    ).display(&mut sys);
    println!("created rectangle 3");

    Rectangle::init(
        26*200,
        26*200,
        Vector::init(
            (width/2-26*200/2).try_into().unwrap(),
            (height/2-26*200/2).try_into().unwrap()
        ),
        Color::init_rgb(255,255,255)
    ).display(&mut sys);
    println!("created rectangle 4");

    let rect5_postion = Vector::init(
        width / 2 - (25 * 200) / 2,
        height / 2 - (25 * 200) / 2
    );
    Rectangle::init(
        25*200,
        25*200,
        rect5_postion,
        Color::init_rgb(183, 65, 14)
    ).display(&mut sys);
    println!("created rectangle 5");

    Circle::init(
        10*200,
        Vector::init(
            (width/2).try_into().unwrap(),
            (height/2).try_into().unwrap()
        ),
        Color::init_rgb(255,255,255)
    ).display(&mut sys);
    println!("created circle 1");


    Circle::init(
        9*200,
        Vector::init(
            (width/2).try_into().unwrap(),
            (height/2).try_into().unwrap()
        ),
        Color::init_rgb(0,0,0)
    ).display(&mut sys);
    println!("created circle 2");

    Circle::init(
        8*200,
        Vector::init(
            (width/2).try_into().unwrap(),
            (height/2).try_into().unwrap()
        ),
        Color::init_rgb(183, 65, 14)
    ).display(&mut sys);
    println!("created circle 3");

    Circle::init(
        7*200,
        Vector::init(
            (width/2).try_into().unwrap(),
            (height/2).try_into().unwrap()
        ),
        Color::init_rgb(0,0,0)
    ).display(&mut sys);
    println!("created circle 4");

    Circle::init(
        6*200,
        Vector::init(
            (width/2).try_into().unwrap(),
            (height/2).try_into().unwrap()
        ),
        Color::init_rgb(255,255,255)
    ).display(&mut sys);
    println!("created circle 5");

    let (img_width,img_height) = (2000,2000);

    Image::init(
        "rust_logo.png".to_string(),
        Vector::init(
            width/2-img_width/2,
            height/2-img_height/2
        )
    ).unwrap().display(&mut sys);
    println!("loaded rust logo");
    let padding = 10;
    Text::init(
        "Rust Logo".to_string(),
        "OpenSans.ttf".to_string(),
        600.0,600.0,
        rect5_postion.add(padding,padding),
        Color::init_rgb(255,255,255)
    ).unwrap().display(&mut sys);

    sys.image("gradient.png");
    println!("created the image");

    sys.ascii(30,"##".to_string());
}
