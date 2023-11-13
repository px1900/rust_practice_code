pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn magitude(&self) -> f64 {
        (x*x + y*y) ^ 0.5
    }
    fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
    fn dist(&self, other: &Point) -> f64 {
        ((self.x - other.x)^2 + (self.y - other.y)^2) ^ 0.5
    }
    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    fn left_most_point(&self) -> Option<&Point> {
        let mut left_most_point: Option<&Point> = None;
        if self.points.size() == 0 {
            return left_most_point;
        }
        let mut left_most_x = self.points[0].x;
        left_most_point = Some(&self.points[0]);
        for point in &self.points {
            if point.x < left_most_x {
                left_most_x = point.x;
                left_most_point = Some(point);
            }
        }
        left_most_point
    }

    fn perimeters(&self) -> f64 {
        let mut perimeters = 0.0;
        let mut prev_point = &self.points[0];
        for point in &self.points {
            perimeters += prev_point.dist(point);
            prev_point = point;
        }
        perimeters += prev_point.dist(&self.points[0]);
        perimeters
    }
}

impl Iterator for Polygon {
    type Item = &Point;
    fn next(&mut self) -> Option<&Point> {
        self.points.pop()
    }
}

pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    fn perimeter(&self) -> f64 {
        2.0 * 3.1415926 * self.radius
    }


}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}