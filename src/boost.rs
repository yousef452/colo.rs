#[derive(Clone, Copy)]
pub struct Vector {
    x : isize,
    y : isize,
}

impl Vector {
    pub fn init(
        x : isize,
        y : isize,
    ) -> Self {
        Self {
            x,
            y
        }
    }
    pub fn get_axis(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn is_equle(&self, x : isize, y : isize) -> bool {
        return self.x == x && self.y == y;
    }
    pub fn is_equle_v(&self,v : Vector) -> bool {
        return self.x == v.x && self.y == v.y;
    }

    pub fn get_y(&self) -> isize {
        self.y
    }
    pub fn get_x(&self) -> isize {
        self.x
    }

    pub fn set_y(&mut self, y : isize) {
        self.y = y;
    }
    pub fn set_x(&mut self, x : isize) {
        self.x = x;
    }

    pub fn change_y(&mut self, y : isize) {
        self.y += y;
    }
    pub fn change_x(&mut self, x : isize) {
        self.x += x;
    }

    pub fn set_axis(&mut self, x : isize, y : isize) {
        self.x = x;
        self.y = y;
    }

    pub fn set_axis_v(&mut self,v : Vector) {
        self.x = v.x;
        self.y = v.y;
    }

    pub fn change_axis(&mut self, x : isize, y : isize) {
        self.x += x;
        self.y += y;
    }

    pub fn change_axis_v(&mut self,v : Vector) {
        self.x += v.x;
        self.y += v.y;
    }
}

#[derive(Debug)]
pub enum SystemErrors {
    OutIndex,
}

pub struct System {
    height : usize,
    width  : usize,
    pixels : Vec<Vec<Color>>,
}

impl System {
    pub fn init_white(
        height : usize,
        width  : usize,
    ) -> Self {
        let col = vec![Color::init(255,255,255,255);width];
        let pixels = vec![col.clone();height];

        Self {
            height,
            width,
            pixels
        }
    }

    pub fn init(
        height : usize,
        width  : usize,
        color  : Color
    ) -> Self {
        let col = vec![color;width];
        let pixels = vec![col.clone();height];

        Self {
            height,
            width,
            pixels
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get_pixel(&self,x : usize,y : usize) -> Result<Color,SystemErrors> {
        if x > self.width || y > self.width {
            Err(SystemErrors::OutIndex)
        }
        else {
            Ok(self.pixels[y][x].clone())
        }
    }

    pub fn insert(&mut self,c : Color,v : Vector) {
        let (x, y) = v.get_axis(); 
        if (x >= 0 && x < self.width.try_into().unwrap()) && (y >= 0 && y < self.height.try_into().unwrap()) {
            self.pixels[y as usize][x as usize] = c;
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    red   : u8,
    blue  : u8,
    green : u8,
    alpha : u8,
}

impl Color {
    pub fn init(
        red   : u8,
        blue  : u8,
        green : u8,
        alpha : u8,
    ) -> Self {
        Self {
            red,
            blue,
            green,
            alpha
        }
    }

    pub fn init_rgb(
        red   : u8,
        blue  : u8,
        green : u8,
    ) -> Self {
        Self {
            red,
            blue,
            green,
            alpha : 255,
        }
    }
    pub fn set_rgb(
        &mut self,
        red   : u8,
        blue  : u8,
        green : u8,
    ) {
        self.red   = red;
        self.green = green;
        self.blue  = blue;
    }

    pub fn set_rgba(
        &mut self,
        red   : u8,
        blue  : u8,
        green : u8,
        alpha : u8,
    ) {
        self.red   = red;
        self.green = green;
        self.blue  = blue;
        self.alpha = alpha;
    }

    pub fn get_rgb(&self) -> (u8, u8, u8) {
        return (self.red, self.green, self.blue)
    }

    pub fn get_rgba(&self) -> (u8, u8, u8,u8) {
        return (self.red, self.green, self.blue,self.alpha)
    }
}
pub trait Shape {
    fn display(&mut self,sys : &mut System);
}

pub struct Line {
    pub point1 : Vector,
    pub point2 : Vector,
    pub color  : Color,

    vectors : Vec<Vector>
}

impl Line {
    pub fn init(
        point1 : Vector,
        point2 : Vector,
        color  : Color
    ) -> Self {
        Self {
            point1,
            point2,
            color,
            vectors : vec![]
        }
    }
    pub fn get_vectors(&self) -> Vec<Vector> {
        self.vectors.clone()
    }
}

impl Shape for Line {
    fn display(&mut self, sys: &mut System) {
        let dx = (self.point2.get_x() - self.point1.get_x()).abs();
        let dy = (self.point2.get_y() - self.point1.get_y()).abs();

        let sx = if self.point1.get_x() < self.point2.get_x() { 1 } else { -1 };
        let sy = if self.point1.get_y() < self.point2.get_y() { 1 } else { -1 };

        let mut err = dx - dy;

        let mut working_vec = self.point1.clone();
        self.vectors = vec![];

        loop {
            sys.insert(self.color.clone(), working_vec.clone());
            self.vectors.push(working_vec.clone());


            if working_vec.is_equle_v(self.point2.clone()) {
                break;
            }

            let e2 = 2 * err;

            if e2 > -dy {
                err -= dy;
                working_vec.change_x(sx);
            }
            if e2 < dx {
                err += dx;
                working_vec.change_y(sy);
            }
        }
    }
}

pub struct Tringle {
    pub point1 : Vector,
    pub point2 : Vector,
    pub point3 : Vector,

    pub color : Color
}

impl Tringle {
    pub fn init(
        point1 : Vector,
        point2 : Vector,
        point3 : Vector,
        color  : Color
    ) -> Self {
        Self {
            point1,
            point2,
            point3,
            color
        }
    }
}


/* 
y1 : x1, x2, x3, x4
y2 : x1, x2, x3, x4
y3 : x1, x2, x3, x4
...
height



pixel : 
*/
impl Shape for Tringle {
    fn display(&mut self,sys : &mut System) {
        let mut line1 = Line::init(self.point1.clone(),self.point2.clone(),self.color.clone());
        let mut line2 = Line::init(self.point2.clone(),self.point3.clone(),self.color.clone());
        let mut line3 = Line::init(self.point3.clone(),self.point1.clone(),self.color.clone());

        line1.display(sys);
        line2.display(sys);
        line3.display(sys);

        struct Xiny {
            y : isize,
            xs : Vec<isize>
        }

        let mut xinys: Vec<Xiny> = vec![];
        for y in 0..sys.height {
            let y_isize = y as isize;
            let mut xs: Vec<isize> = vec![];

            for l in [&line1, &line2, &line3] {
                for vec in l.get_vectors() {
                    if vec.y == y_isize {
                        xs.push(vec.get_x());
                    }
                }
            }

            if !xs.is_empty() {
                xs.sort();
                xinys.push(Xiny { y: y_isize, xs });
            }
        }
        for x in xinys {
            if x.xs.len() > 0 {
                Line::init(Vector::init(x.xs[0],x.y),Vector::init(x.xs[x.xs.len()-1],x.y),self.color).display(sys);
            }
        }
    }
}

pub struct Circle {
    radius   : usize,
    position : Vector,
    color    : Color,
}

impl Circle {
    pub fn init(
        radius   : usize,
        position : Vector,
        color    : Color
    ) -> Self {
        Self {
            radius,
            position,
            color
        }
    }
}

impl Shape for Circle {
    fn display(&mut self, sys: &mut System) {
        let mut x = 0;
        let mut y = self.radius as isize;

        let mut p: isize = 1 - self.radius as isize;

        while x <= y {
            let cx = self.position.x;
            let cy = self.position.y;

            for xi in (cx - x)..=(cx + x) {
                sys.insert(self.color, Vector::init(xi, cy + y));
                sys.insert(self.color, Vector::init(xi, cy - y));
            }

            for xi in (cx - y)..=(cx + y) {
                sys.insert(self.color, Vector::init(xi, cy + x));
                sys.insert(self.color, Vector::init(xi, cy - x));
            }

            if p < 0 {
                p += 2 * x + 3;
            }
            else {
                p += 2 * (x - y) + 5;
                y -= 1;
            }
            x += 1;
        }
    }
}

