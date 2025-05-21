// implementation is in the impl block
// implementation of the enum
// enum to represent different shapes
enum Shape {
    Circle(f64),       // Circle with radius
    Square(f64),       // Square with side length
    Rectangle(f64, f64), // Rectangle with width and height
}

// struct to represent a shape (if needed)
struct ShapeStruct {
    shape: Shape,
}
impl Shape {
    // method to calculate the area of the shape
    fn area(&self) -> f64 { //here self is a reference to the enum , self is used to refer to the instance of the enum
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Rectangle(width, height) => width * height,
        }
    }
}
impl ShapeStruct {
    // method to create a new ShapeStruct
    fn new(shape: Shape) -> Self {
        ShapeStruct { shape }
    }
}
impl ShapeStruct {
    // method to calculate the area of the shape
    fn area(&self) -> f64 {
        match self.shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(side) => side * side,
            Shape::Rectangle(width, height) => width * height,
        }
    }
}
fn main() {
    // create a circle with radius 5.0
    let circle = Shape::Circle(5.0);
    // create a square with side length 4.0
    let square = Shape::Square(4.0);
    // create a rectangle with width 3.0 and height 6.0
    let rectangle = Shape::Rectangle(3.0, 6.0);

    // print the area of each shape
    println!("Area of Circle: {}", circle.area());
    println!("Area of Square: {}", square.area());
    println!("Area of Rectangle: {}", rectangle.area());
}