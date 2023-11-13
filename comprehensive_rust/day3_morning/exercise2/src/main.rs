#[derive(Debug, Copy, Clone, PartialEq, Eq)]

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn magnitude(&self) -> f64 {
        f64::from(self.x*self.x + self.y*self.y).sqrt()
    }
    fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
    fn dist(&self, other: Point) -> f64 {
        f64::from((self.x - other.x).pow(2) + (self.y - other.y).pow(2)).sqrt()
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}



pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new() -> Polygon {
        Polygon {
            points: Vec::new(),
        }
    }

    fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    fn left_most_point(&self) -> Option<Point> {
        let mut left_most_point: Option<Point> = None;
        if self.points.iter().len() == 0 {
            return left_most_point;
        }
        let mut left_most_x = self.points[0].x;
        left_most_point = Some(self.points[0]);
        for point in &self.points {
            if point.x < left_most_x {
                left_most_x = point.x;
                left_most_point = Some(*point);
            }
        }
        left_most_point
    }

    fn perimeters(&self) -> f64 {
        let mut perimeters = 0.0;
        let mut prev_point = &self.points[0];
        for point in &self.points {
            perimeters += prev_point.dist(*point);
            prev_point = point;
        }
        perimeters += prev_point.dist(self.points[0]);
        perimeters
    }
}


pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    fn new(center: Point, radius: i32) -> Circle {
        Circle {
            center: center,
            radius: radius,
        }
    }
    fn perimeter(&self) -> f64 {
        2.0 * 3.1415926 * f64::from(self.radius)
    }


}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Circle> for Shape {
    fn from(circle: Circle) -> Shape {
        Shape::Circle(circle)
    }
}

impl From<Polygon> for Shape {
    fn from(polygon: Polygon) -> Shape {
        Shape::Polygon(polygon)
    }
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(polygon) => polygon.perimeters(),
            Shape::Circle(circle) => circle.perimeter(),
        }
    }
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