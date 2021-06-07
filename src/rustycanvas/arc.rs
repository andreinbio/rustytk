use super::point::Point;

#[derive(Debug, Copy, Clone, Default)]
pub struct Arc {
    x: u32,
    y: u32,
    radius: u32,
    start_angle: u32,
    end_angle: u32,
    anticlockwise: bool,
}

impl Arc {
    pub fn new(x: u32, y: u32, radius: u32, start_angle: u32, end_angle: u32, anticlockwise: bool) -> Self {
        Arc {x, y, radius, start_angle, end_angle, anticlockwise}
    }

    pub fn get_start_point(&self) -> Point {
        Point::default()
    }

    pub fn get_end_point(&self) -> Point {
        Point::default()
    }
}
