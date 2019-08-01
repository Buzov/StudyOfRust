#[derive(Default)]
pub(crate) struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point{x: 0.0, y: 0.0}
    }

    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("Point x: {} y: {}", self.x, self.y);
    }
}

pub(crate) fn test_partial_eq() {
    let p1 = Point::default();
    let p2 = Point::default();
    let p1_eq_p2 = p1 == p2;
    let p3 = Point{x: 1.0, y: 2.0};
    let p1_eq_p3 = p1 == p3;
    println!("p1 == p2: {}", p1_eq_p2);
    println!("p1 == p2: {}", p1_eq_p3);
}

pub(crate) fn test_point() {
    let point = Point{x: 1.0, y: 2.0};
    println!("point  distance_from_origin: {}", point.distance_from_origin());
    let point_origin = Point::origin();
    println!("point_origin  distance_from_origin: {}", point_origin.distance_from_origin());
//    let point = Point{x: 1.0, y: 2.0, };
}