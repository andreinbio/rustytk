mod math;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    start: Point,
    end: Point,
}

#[derive(Debug)]
pub struct Path2D {
    current_position: Point,
    vectors: Vec<Vector>,
}

#[derive(Debug)]
pub struct Data {
    pub points: Vec<[u32;2]>,
    pub color: [f32;4],
}

#[derive(Debug)]
pub struct CanvasApi {
    path: Option<Path2D>,
    pub data: Vec<Data>,
    fill_style: [f32;4],
    stroke_style: [f32;4],
    // coordinates conversion
    width: u32, // @TODO not used
    height: u32, // @TODO not used
    depth: f32, // @TODO not used
    top_left: [f32;2], // @TODO not used
}



impl Path2D {
    pub fn new() -> Self {
        Path2D {
            current_position: Point {x: 0, y: 0},
            vectors: vec![],
        }
    }
    /// Paths
    pub fn add_path() {
        unimplemented!("add_path");
    }

    /// Add the last line between the last provided point and the start point from the first vector
    pub fn close_path(&mut self) {
        let start_point: Point = self.vectors.get(0).cloned().unwrap_or(Vector {
            start: Point {x: 0, y: 0},
            end: Point {x: 0, y: 0},
        }).start;

        // do nothing if path was closed already
        // path is closed if start_point == current_possition
        if self.current_position.x != start_point.x || self.current_position.y != start_point.y {
            self.line_to(start_point.x, start_point.y);
        }
    }

    /// Update current position with the new coordinates
    pub fn move_to(&mut self, x: u32, y: u32) {
        self.current_position = Point {x: x, y: y};
    }

    /// Add a new line based on current position and provided coordinates
    pub fn line_to(&mut self, x: u32, y: u32) {
        let point_0: Point = Point {
            x: self.current_position.x,
            y: self.current_position.y,
        };
        let point_1: Point = Point {
            x: x,
            y: y,
        };
        let vector = Vector {
            start: point_0,
            end: point_1,
        };
        self.vectors.push(vector);

        // update current position with provided coordinates
        self.move_to(x, y);
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
            depth: 2.0, // @TODO not used
            top_left: [-1.0, 1.0], // @TODO not used
            width: width, // @TODO not used
            height: height, // @TODO not used
            path: None,
            data: vec![],
            fill_style: [0.0, 0.0, 0.0, 1.0],
            stroke_style: [0.0, 0.0, 0.0, 1.0],
        }
    }

    /// Paths
    pub fn begin_path(&mut self) {
        // create a new path
        self.path = Some(Path2D::new());
        // reset the fill and stroke colors
        self.fill_style = [0.0, 0.0, 0.0, 1.0];
        self.stroke_style = [0.0, 0.0, 0.0, 1.0];
    }

    pub fn close_path(&mut self) {
        self.path.as_mut().map(|path| path.close_path());
    }

    pub fn move_to(&mut self, x: u32, y: u32) {
        self.path.as_mut().map(|path| path.move_to(x, y));
    }

    pub fn line_to(&mut self, x: u32, y: u32) {
        self.path.as_mut().map(|path| path.line_to(x, y));
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
    pub fn fill(&mut self) {
        // close path it it was not closed yet
        self.close_path();

        // get bounding box
        let mut x_min = 0;
        let mut y_min = 0;
        let mut first_loop: bool = true;

        let mut x_max = 0;
        let mut y_max = 0;

        // prepare the points array
        let mut points: Vec<[u32;2]> = vec![];

        let path: Path2D = self.path.take().unwrap_or(Path2D::new());

        // iterate over the vectors and get the bounding box
        // @TODO maybe to get the bounding box during lines construction ?
        for vector in path.vectors.iter() {
            let vector_x_min = std::cmp::min(vector.start.x, vector.end.x);
            let vector_y_min = std::cmp::min(vector.start.y, vector.end.y);

            let vector_x_max = std::cmp::max(vector.start.x, vector.end.x);
            let vector_y_max = std::cmp::max(vector.start.y, vector.end.y);

            if first_loop {
                x_min = vector_x_min;
                y_min = vector_y_min;
                first_loop = false;
            } else {
                x_min = std::cmp::min(x_min, vector_x_min);
                y_min = std::cmp::min(y_min, vector_y_min);
            }

            x_max = std::cmp::max(x_max, vector_x_max);
            y_max = std::cmp::max(y_max, vector_y_max);
        }

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if math::wn_pnpoly(&Point {x: x, y: y}, &path.vectors) > 0 {
                    points.push([x, y]);
                }
            }
        }

        self.data.push(Data {
            points: points,
            color: self.fill_style,
        });
    }

    pub fn stroke(&mut self) {
        // prepare the points array
        let mut points: Vec<[u32;2]> = vec![];

        let path: Path2D = self.path.take().unwrap_or(Path2D::new());

        // get each vector
        for vector in path.vectors.iter() {
            let point_0 = vector.start;
            let point_1 = vector.end;

            let x_min = std::cmp::min(point_0.x, point_1.x);
            let y_min = std::cmp::min(point_0.y, point_1.y);

            let x_max = std::cmp::max(point_0.x, point_1.x);
            let y_max = std::cmp::max(point_0.y, point_1.y);

            let width = x_max - x_min;
            let height = y_max - y_min;

            if width > height {
                for x in x_min..=x_max {
                    let y = math::get_line_y_point(x, &point_0, &point_1);
                    points.push([x, y]);
                }
            } else {
                for y in y_min..=y_max {

                    let x = math::get_line_x_point(y, &point_0, &point_1);
                    points.push([x, y]);
                }
            }
        }

        self.data.push(Data {
            points: points,
            color: self.stroke_style,
        });
    }

    /// Fill and stroke styles
    pub fn fill_style(&mut self, color: [f32;4]) {
        self.fill_style = color;
    }

    pub fn stroke_style(&mut self, color: [f32; 4]) {
        self.stroke_style = color;
    }
}
