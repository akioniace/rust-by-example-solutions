#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32,f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// solution

fn rect_area(r: Rectangle) -> f32 {
    let (tx, ty) = (r.top_left.x, r.top_left.y);
    let (bx, by) = (r.bottom_right.x, r.bottom_right.y);

    let height = ty - by;
    let width = bx - tx;

    height * width
}

// solution

fn square(p: Point, len: f32) -> Rectangle {
    return Rectangle {
        top_left: Point {
            x: p.x,
            y: p.y + len,
        },
        bottom_right: Point {
            x: p.x + len,
            y: p.y,
        },
    }
}

fn main() {
    let name = String::from("Peter");
    let age =27;
    let peter = Person{name,age};

    println!("{:?}", peter);

    let point: Point = Point {x: 10.3, y: 0.4};

    println!("point coordinates: ({}, {})",point.x, point.y);

    let bottom_right = Point{ x: 5.2, ..point};

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: top_edge, y: left_edge },
        bottom_right: bottom_right,
    };
    println!("rect: {:?}", _rectangle);

    println!("Rectangle Area: {}", rect_area(_rectangle));

    let _unnit = Unit;

    let pair = Pair(1,0.01);

    println!("pair contain {:?} and {:?}", pair.0,pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains: {:?} and {:?}", integer, decimal);

    let s = square(Point{x:2.0,y:2.0}, 4.0);

    println!("Square: {:?}", s);

}