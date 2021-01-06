
#[derive(Debug)]
pub struct Path2D {

}

#[derive(Debug)]
pub struct CanvasApi {
    paths: Vec<Path2D>,
    fill_style: [f32;3],
    stroke_style: [f32;3],
    // coordinates conversion
    width: u32,
    height: u32,
    depth: f32,
    top_left: [f32;2],
    points: Vec<f32>,
}

impl Path2D {
    pub fn new() -> Self {
        Path2D {

        }
    }
    /// Paths
    pub fn add_path() {
        unimplemented!("add_path");
    }

    pub fn close_path() {
        unimplemented!("close_path");
    }

    pub fn move_to() {
        unimplemented!("move_to");
    }

    pub fn line_to() {
        unimplemented!("line_to");
    }

    pub fn bezier_curve_to() {
        unimplemented!("bezier_curve_to");
    }

    pub fn quadratic_curve_to() {
        unimplemented!("quadratic_curve_to");
    }

    pub fn arc() {
        unimplemented!("arc");
    }

    pub fn arc_to() {
        unimplemented!("arc_to");
    }

    pub fn ellipse() {
        unimplemented!("ellipse");
    }

    pub fn rect() {
        unimplemented!("rect");
    }
}

impl CanvasApi {
    pub fn new(width: u32, height: u32) -> Self {
        CanvasApi {
            paths: vec![],
            fill_style: [0.0, 0.0, 0.0],
            stroke_style: [0.0, 0.0, 0.0],
            depth: 2.0,
            top_left: [-1.0, 1.0],
            width: width,
            height: height,
            points: vec![],
        }
    }

    /// Paths
    pub fn begin_path(&mut self) {
        self.paths.push(Path2D::new());
    }

    pub fn close_path() {
        unimplemented!("close_path");
    }

    pub fn move_to() {
        unimplemented!("move_to");
    }

    pub fn line_to() {
        unimplemented!("line_to");
    }

    pub fn bezier_curve_to() {
        unimplemented!("bezier_curve_to");
    }

    pub fn quadratic_curve_to() {
        unimplemented!("quadratic_curve_to");
    }

    pub fn arc() {
        unimplemented!("arc");
    }

    pub fn arc_to() {
        unimplemented!("arc_to");
    }

    pub fn ellipse() {
        unimplemented!("ellipse");
    }

    pub fn rect() {
        unimplemented!("rect");
    }

    /// Drawing Paths
    pub fn fill() {
        unimplemented!("fill");
    }

    pub fn stroke() {
        unimplemented!("stroke");
    }
}
