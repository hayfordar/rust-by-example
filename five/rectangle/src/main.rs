#[derive(Debug)]
struct Rectangle<T> {
    h : T,
    w : T
}

// This is a naive approach, we can create a method for Rectangle
//fn rectangle_area(rectangle : &Rectangle<i32>) -> i32 {
//    rectangle.h * rectangle.w
//}

// Another primitive approach, we can further template-ize this and
// create a cleaner API
impl Rectangle<i32> {
    fn area(&self) -> i32 {
        self.h * self.w
    }

    fn can_fit(&self, rhs : &Rectangle<i32>) -> bool {
        (self.h > rhs.h) && (self.w > rhs.w)
    }

    // Associated function not bound to instance of Rectangle
    // These are largely used for constructors
    fn square(dim : i32) -> Rectangle<i32> {
        Rectangle { h : dim, w : dim }
    }
}


fn main() {
    let rect : Rectangle<i32> = Rectangle { h : 32, w : 100 };
    //println!("Area of {:?} is {:?}", rect, rectangle_area(&rect);
    println!("Area of {:?} is {:?}", rect, rect.area());

    let mut rect2 : Rectangle<i32> = Rectangle { h : 10, w : 10 };
    println!("rect can fit rect2 : {:?}", rect.can_fit(&rect2));
    println!("rect2 can fit rec2 : {:?}", rect2.can_fit(&rect));
    rect2.h = 33;
    rect2.w = 101;
    println!("rect2 can fit rect after growth : {:?}", rect2.can_fit(&rect));
}
