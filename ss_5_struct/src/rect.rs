struct Rect{
    width:f64,
    height:f64,
}

impl Rect{
    fn new(width:f64,height:f64)->Rect{
        Rect{
            width,
            height,
        }
    }

    fn cal_area(&self)->f64{
        self.width * self.height
    }
}

pub fn run(){
    let rect = Rect::new(2.0,1.9);
    println!("Area of rect{}",rect.cal_area());
}