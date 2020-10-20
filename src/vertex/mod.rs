use winit::{window::Window};

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

        dbg!((position, scale_factor));

        let start_pos = (position.0, position.1);
        let width = 10 as i32;
        let height = 10 as i32;
        let background = [1.0, 0.0, 0.0];
        // let mut vec: Vec<Vertex> = Vec::with_capacity((width * height) as usize);

        // for y in 0..height {
        //     for x in 0..width {
        //         vec.push(Vertex {
        //             position: [((start_pos.0 + x) as f64) * scale_factor,  ((start_pos.1 + y) as f64) * scale_factor, 0.0],
        //             color: background,
        //         });
        //     }
        // }

        // vec.push(Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] });
        // vec.push(Vertex{
        //     position: [0.001, 0.002, 0.0],
        //     color: background
        // });
        // vec.push(Vertex{
        //     position: [0.001, 0.003, 0.0],
        //     color: background
        // });
        // vec.push(Vertex{
        //     position: [0.001, 0.004, 0.0],
        //     color: background
        // });
        // vec.push(Vertex{
        //     position: [0.001, 0.005, 0.0],
        //     color: background
        // });
        // vec.push(Vertex{
        //     position: [0.001, 0.006, 0.0],
        //     color: background
        // });
        // vec.push(Vertex{
        //     position: [0.001, 0.001, 0.0],
        //     color: background
        // });
        // vec.push(Vertex{
        //     position: [0.001, 0.002, 0.0],
        //     color: background
        // });
        // vec.push(Vertex{
        //     position: [0.001, 0.003, 0.0],
        //     color: background
        // });
        // vec.push(Vertex{
        //     position: [0.001, 0.004, 0.0],
        //     color: background
        // });
        // vec.push(Vertex{
        //     position: [0.001, 0.005, 0.0],
        //     color: background
        // });
        // vec.push(Vertex{
        //     position: [0.001, 0.006, 0.0],
        //     color: background
        // });

        let vec: Vec<Vertex> = vec![
            Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
            Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
            Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
            Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
            Vertex { position: [0.44147372, 0.2347359, 0.0],color: [0.5, 0.0, 0.5] }, // E
        ];
        // dbg!(&vec);
        vec
    }
}
