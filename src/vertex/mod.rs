use winit::{window::Window};

use crate::rustycanvas::{CanvasApi,Path2D};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32;3],
    pub color: [f32;4],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32;3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float4,
                },
            ]
        }
    }

    pub fn get_vertices_slice(window: &Window) -> Vec<Vertex> {
        let position = if let Ok(pos) = window.inner_position() {
            (pos.x, pos.y)
        } else {
            (0, 0)
        };
        let scale_factor: f64 = window.scale_factor();
        let inner_size = window.inner_size();

        let mut canvas = CanvasApi::new(inner_size.width, inner_size.height);
        // let mut path = Path2D::new();

        // Filled triangle
        canvas.begin_path();
        canvas.move_to(25, 25);
        canvas.line_to(105, 25);
        canvas.line_to(25, 105);
        canvas.fill_style([0.0, 1.0, 0.0, 1.0]);
        canvas.fill();

        canvas.begin_path();
        canvas.move_to(30, 30);
        canvas.line_to(110, 30);
        canvas.line_to(30, 110);
        canvas.fill_style([1.0, 0.0, 0.0, 1.0]);
        canvas.fill();

        // Stroked triangle
        canvas.begin_path();
        canvas.move_to(125, 125);
        canvas.line_to(125, 45);
        canvas.line_to(45, 125);
        canvas.close_path();
        canvas.stroke();

        let mut vec: Vec<Vertex> = vec![];

        for data in canvas.data.iter() {
            let color = data.color;

            for point in data.points.iter() {
                let x_pos = get_x_pos(point[0], inner_size.width);
                let y_pos = get_y_pos(point[1], inner_size.height);
                vec.push(Vertex {
                    position: [x_pos, y_pos, 0.0],
                    color: color,
                });
            }
        }

        vec
    }
}


fn get_x_pos(client_x: u32, w_width: u32) -> f32 {
    let depth = 2.0;
    let top = (-1.0, 1.0);
    // some cool math here !!!
    // user coordonate system starts frop 0,0 from top left corner
    // translate user position to window position
    let x_percent = client_x as f32 * 100.0 / w_width as f32;

    let x_diff = depth * x_percent / 100.0;

    let x_pos = top.0 + x_diff;

    x_pos
}

fn get_y_pos(client_y: u32, w_height: u32) -> f32 {
    let depth = 2.0;
    let top = (-1.0, 1.0);
    // some cool math here !!!
    // user coordonate system starts frop 0,0 from top left corner
    // translate user position to window position
    let y_percent = client_y as f32 * 100.0 / w_height as f32;

    let y_diff = depth * y_percent / 100.0;

    let y_pos = top.1 - y_diff;

    y_pos
}
