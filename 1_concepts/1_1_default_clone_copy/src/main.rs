#[derive(Clone, Copy, Default, Debug)]
struct Point {
    pub x:  f32,
    pub y:  f32
}

#[derive(Clone, Debug)]
struct Polyline {
    pub points: Vec<Point>
}

impl Polyline {
    fn new(p: Point) ->Self {
        Self { points: vec![p] }
    }

    pub fn add_point(& mut self, p_add: Point) {
        self.points.push(p_add) ;
    }
}

fn main() {
    /*
    use smart_default::SmartDefault;
    #[derive(Clone, Copy, Debug, SmartDefault)]
    struct Point_1 {
        #[default = 0.1]
        x:  f32,
        y:  f32,
        #[default = 100.]
        z:  f32
    }

    let p1 = Point_1{y: 10., ..Default::default()} ;
    println!("p1: {:?}", p1) ;
     */

    let p = Point::default() ;
    println!("p: {:?}", p) ;
    let mut p_new = p ;
    println!("p: {:?}, p_new: {:?}", p, p_new) ;

    let mut p_lines = Polyline::new(p) ;
    p_new.x += 10. ;
    p_new.y += 20. ;

    p_lines.add_point(p_new);
    println!("p_lines: {:?}", p_lines) ;
}