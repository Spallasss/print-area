// 圆形
struct Circle {
    radius:f64,
}
// 长方形
struct Rectangle {
    long:u64,
    width:u64,  
}

// 定义可以打印面积的trait
pub trait HasArea {
    fn print_area(&self);
}

// 为图形实现print_area
impl HasArea for Circle {
    fn print_area(&self) {
        let circle_area = std::f64::consts::PI * (self.radius * self.radius);
        println!("circle area is: {}", circle_area);
    }
}
impl HasArea for Rectangle {
    fn print_area(&self) {
        let rectangle_area = self.long * self.width;
        println!("rectangle area is: {}", rectangle_area);
    }
}

//T为实现了可打印面积特征的图形
fn area_print<T:HasArea> (item:T) {
    item.print_area();
}

fn main() {
    let circle = Circle {
        radius:9.9,
    };
    let rectangle = Rectangle{
        long:100,
        width:20,
    };
    area_print(circle);
    area_print(rectangle);
}
