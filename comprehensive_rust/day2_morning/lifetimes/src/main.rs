#[derive(Debug)]
struct Point(i32, i32);

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 { p1 } else { p2 }
}

fn main() {
    let p1: Point = Point(10, 10);
    let p3: &Point;
    {
        let p2: Point = Point(20, 20);
        p3 = left_most(&p1, &p2);
        println!("p3: {p3:?}");
    }
    //! If we uncomment the following line, we will get a compile error
    //! The reason is that p1, p2, p3 should be alive when any of them is used
    // println!("p3: {p3:?}");
}