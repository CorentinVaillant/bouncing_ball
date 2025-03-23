use glium::implement_vertex;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 4],
}

impl From<[f32; 4]> for Vertex {
    fn from(position: [f32; 4]) -> Self {
        Self { position }
    }
}

impl From<[f32; 2]> for Vertex {
    fn from(value: [f32; 2]) -> Self {
        let [x, y] = value;
        Self {
            position: [x, y, 0., 1.],
        }
    }
}

implement_vertex!(Vertex, position);
