use winit::{window::Window};

use crate::rustycanvas::CanvasApi;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
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
        // dbg!(&canvas);
        // dbg!(canvas.begin_path());

        canvas.line_to(10, 20);

        // dbg!((position, scale_factor, inner_size));

        let depth = 2.0;
        let top = (-1.0, 1.0);
        let start_pos = (10, 10);

        let width = 100 as i32;
        let height = 250 as i32;
        let background = [1.0, 0.0, 0.0];

        // some cool math here !!!
        // user coordonate system starts frop 0,0 from top left corner
        // translate user position to window position
        let x_percent = start_pos.0 as f32 * 100.0 / inner_size.width as f32;
        let y_percent = start_pos.1 as f32 * 100.0 / inner_size.height as f32;

        let x_diff = depth * x_percent / 100.0;
        let y_diff = depth * y_percent / 100.0;

        let x_pos = top.0 + x_diff;
        let y_pos = top.1 - y_diff;

        // end cool math stuff :)

        // let mut vec: Vec<Vertex> = Vec::with_capacity((width * height) as usize);
        let mut vec: Vec<Vertex> = vec![];

        // vec.push(Vertex{
        //     position: [x_pos, y_pos, 0.0],
        //     color: background,
        // });

        for y in 0..height {
            for x in 0..width {
                let x_pos = get_x_pos(x, inner_size.width);
                let y_pos = get_y_pos(y, inner_size.height);
                vec.push(Vertex {
                    position: [x_pos, y_pos, 0.0],
                    color: background,
                });
            }
        }


        // vec.push(Vertex{
        //     position: [1.0, 0.0, 0.0],
        //     color: background,
        // });
        // vec.push(Vertex{
        //     position: [1.0, -1.0, 0.0],
        //     color: background,
        // });
        // vec.push(Vertex{
        //     position: [-1.0, 1.0, 0.0],
        //     color: background,
        // });
        // vec.push(Vertex{
        //     position: [0.0, 0.0, 0.0],
        //     color: [0.0, 1.0, 0.0],
        // });

        // let vec: Vec<Vertex> = vec![
        //     Vertex { position: [0.0, 0.0, 0.0], color: background, },
        // ];

        // let vec: Vec<Vertex> = vec![
            // Vertex { position: [-0.0868241, -0.49240386, 0.0], color: [1.0, 1.0, 1.0] }, // A
            // Vertex { position: [1.0, 0.0, 0.0], color: [1.0, 1.0, 1.0] }, // A
            // Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [1.0, 0.0, 1.0] }, // B
            // Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
            // Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
            // Vertex { position: [0.44147372, 0.2347359, 0.0],color: [0.5, 0.0, 0.5] }, // E
        // ];
        // dbg!(&vec);
        vec
    }
}


fn get_x_pos(client_x: i32, w_width: u32) -> f32 {
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

fn get_y_pos(client_y: i32, w_height: u32) -> f32 {
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
