use super::point::Point;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vector {
    pub start: Point,
    pub end: Point,
}
