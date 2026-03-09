#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

//pub const VERTICES: &[Vertex] = &[
//    // Front face
//    Vertex {
//        position: [-0.5, -0.5, 0.5],
//        color: [0.9, 0.2, 0.2],
//    }, // 0
//    Vertex {
//        position: [0.5, -0.5, 0.5],
//        color: [0.2, 0.9, 0.2],
//    }, // 1
//    Vertex {
//        position: [0.5, 0.5, 0.5],
//        color: [0.2, 0.2, 0.9],
//    }, // 2
//    Vertex {
//        position: [-0.5, 0.5, 0.5],
//        color: [0.9, 0.9, 0.2],
//    }, // 3
//    // Back face
//    Vertex {
//        position: [-0.5, -0.5, -0.5],
//        color: [0.9, 0.2, 0.9],
//    }, // 4
//    Vertex {
//        position: [0.5, -0.5, -0.5],
//        color: [0.2, 0.9, 0.9],
//    }, // 5
//    Vertex {
//        position: [0.5, 0.5, -0.5],
//        color: [0.9, 0.5, 0.2],
//    }, // 6
//    Vertex {
//        position: [-0.5, 0.5, -0.5],
//        color: [0.5, 0.2, 0.9],
//    }, // 7
//];
//
//pub const INDICES: &[u16] = &[
//    0, 1, 2, 2, 3, 0, // front
//    5, 4, 7, 7, 6, 5, // back
//    4, 0, 3, 3, 7, 4, // left
//    1, 5, 6, 6, 2, 1, // right
//    3, 2, 6, 6, 7, 3, // top
//    4, 5, 1, 1, 0, 4, // bottom
//];

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
