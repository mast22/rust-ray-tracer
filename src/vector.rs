#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(&x: &f64, &y: &f64, &z: &f64) -> Point {
        Point { x, y, z }
    }
}

// В конце концов Вектор - это оберка над точкой
pub type Vector = Point;


// Однако вектор требует дополнительного функционала для операций с ним