#![allow(dead_code)]


use glium::{
    DepthTest, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer, backend::Facade,
    dynamic_uniform, index::PrimitiveType,
};

use crate::{
    traits::{self, CanvasDrawable, Drawable},
    vertex::Vertex,
};

pub struct Canvas {
    pub data: CanvasData,
    pub elements: Vec<Box<dyn CanvasDrawable>>,

    program: Program,

    z: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct CanvasData {
    pub size: (f32, f32),
    pub position: (f32, f32), //%

    pub window_resolution: (u32, u32),
}

impl Canvas {
    pub fn new(position: (f32, f32), program: Program) -> Self {
        Self {
            data: CanvasData {
                position,
                size: (1., 1.),

                window_resolution: (0, 0),
            },
            elements: vec![],
            program,

            z: 0.,
        }
    }

    pub fn push_elem(&mut self, element: Box<dyn CanvasDrawable>) {
        self.elements.push(element);
    }

    pub fn append_elem(&mut self, elems_vec: Vec<Box<dyn CanvasDrawable>>) {
        let mut vec = elems_vec;
        self.elements.append(&mut vec);
    }
}

impl Drawable for Canvas {
    fn draw<F: Facade + Sized>(
        &self,
        facade: &F,
        target: &mut Frame,
    ) -> Result<(), traits::DrawError> {
        let (vert_buff, ind_buff) = self.get_vert_buff(facade).unwrap();

        let mut param = DrawParameters::default();
        // param.depth.test = DepthTest::IfMoreOrEqual;
        param.depth.test = DepthTest::IfMore;

        for elem in &self.elements {
            let mut uniforms = elem.canvas_uniforms();
            let dimension = target.get_dimensions();

            for uni in &mut uniforms {
                uni.add("resolution", &dimension);
                uni.add("canva_z", &self.z);
                uni.add("canva_pos", &self.data.position);
                uni.add("canva_size", &self.data.size);

                target
                    .draw(&vert_buff, &ind_buff, &self.program, uni, &param)
                    .unwrap();
            }
        }

        Ok(())
    }
}

impl CanvasDrawable for Canvas {
    fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    fn get_z(&self) -> f32 {
        self.z
    }

    fn canvas_uniforms(&self) -> Vec<glium::uniforms::DynamicUniforms> {
        vec![dynamic_uniform! {
            canva_z : &self.z,
            canva_pos:&self.data.position,
            canva_size:&self.data.size,
        }]
    }

    fn update(&mut self, _canva_info: &CanvasData, dt: f32) {
        for elem in &mut self.elements {
            elem.update(&self.data, dt);
        }
    }

    fn on_window_resized(&mut self, new_size: (u32, u32)) {
        self.data.window_resolution = new_size;
        for elem in &mut self.elements {
            elem.on_window_resized(new_size);
        }
    }

    fn on_drag(&mut self, old_pos: [f32; 2], new_pos: [f32; 2]) {
        for elem in &mut self.elements {
            if elem.is_absolute_coord_in(old_pos.into()) {
                elem.on_drag(old_pos, new_pos);
            }
        }
    }

    fn is_absolute_coord_in(&self, coord: (f32, f32)) -> bool {
        0. <= coord.0
            && coord.0
                <= self.data.position.0 + self.data.size.0 * self.data.window_resolution.0 as f32
            && 0. <= coord.1
            && coord.1
                <= self.data.position.1 + self.data.size.1 * self.data.window_resolution.1 as f32
    }

    fn is_relative_coord_in(&self, coord: (f32, f32)) -> bool {
        0. <= coord.0
            && coord.0 <= self.data.position.0 + self.data.size.0
            && 0. <= coord.1
            && coord.1 <= self.data.position.1 + self.data.size.1
    }

    fn on_click(&mut self, coord: (f32, f32)) {
        for elem in &mut self.elements {
            if elem.is_absolute_coord_in(coord) {
                elem.on_click(coord);
            }
        }
    }

    fn on_click_release(&mut self) {
        for elem in &mut self.elements {
            elem.on_click_release();
        }
    }

    fn on_window_moved(&mut self, new_pos: (f32, f32)) {
        for elem in &mut self.elements {
            elem.on_window_moved(new_pos);
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum CanvaToVertBuffErr {
    VertexBufferCreationError(glium::vertex::BufferCreationError),
    IndexBufferCreationError(glium::index::BufferCreationError),
    CacheError(glium::buffer::CopyError),
}

impl Canvas {
    pub fn get_vert_buff<F: Facade>(
        &self,
        facade: &F,
    ) -> Result<(VertexBuffer<Vertex>, IndexBuffer<u16>), CanvaToVertBuffErr> {
        let x_i = (self.data.position.0 * 2.0) - 1.0;
        let y_i = (self.data.position.1 * 2.0) - 1.0;
        let x_s = ((self.data.position.0 + self.data.size.0) * 2.0) - 1.0;
        let y_s = ((self.data.position.1 + self.data.size.1) * 2.0) - 1.0;

        let vert_buff = VertexBuffer::new(
            facade,
            &[
                [x_s, y_s].into(),
                [x_s, y_i].into(),
                [x_i, y_i].into(),
                [x_i, y_s].into(),
            ],
        )
        .map_err(CanvaToVertBuffErr::VertexBufferCreationError)?;

        let ind_buff = IndexBuffer::new(facade, PrimitiveType::TriangleFan, &[0, 1, 2, 3])
            .map_err(CanvaToVertBuffErr::IndexBufferCreationError)?;

        Ok((vert_buff, ind_buff))
    }
}
