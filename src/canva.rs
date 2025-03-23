#![allow(dead_code)]


use std::cell::RefCell;

use glium::{
    DepthTest, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer, backend::Facade,
    index::PrimitiveType, uniforms::Uniforms, winit::window::Window,
};

pub use crate::draw::Drawable;
use crate::vertex::Vertex;

#[derive(Debug)]
pub struct Canva<E: CanvaDrawable> {
    pub width: f32,
    pub height: f32,
    pub position : (f32,f32), //%

    pub elements: Vec<E>,
    program: Program,

    //cache
    vert_buff :RefCell<Option<VertexBuffer<Vertex>>>,
    inde_buff :RefCell<Option<IndexBuffer<u16>>>,
    z : f32,
}

impl<Drawable: CanvaDrawable> Canva<Drawable> {
    pub fn new(elements: Vec<Drawable>,position:(f32,f32) ,program: Program) -> Self {
        let mut elements = elements;
        let mut z = 0.;
        let len = (0..elements.len()).fold(0., |v, _| v + 1.);
        for elem in &mut elements {
            z += 1.;
            elem.set_z(z / len);
        }
        Self {
            position,
            width : 1.,
            height :1.,

            elements,
            program,

            vert_buff:None.into(),
            inde_buff:None.into(),
            z:0.,
        }

    }

    pub fn add_elem(&mut self, elem: Drawable) {
        self.elements.push(elem);
    }

    
    
}

impl<E:CanvaDrawable> Drawable for Canva<E>{
    type DrawResult = ();

    fn draw<F: Facade + Sized>(&self, facade: &F, target: &mut Frame) {
        let (vert_buff, ind_buff) = self.get_vert_buff(facade).unwrap();

        let mut param = DrawParameters::default();
        param.depth.test = DepthTest::IfMore;


        for uniform in self.to_uniform(target.get_dimensions()){
            target
                .draw(&vert_buff, &ind_buff, &self.program, &uniform, &param)
                .unwrap();
        }
    }
}

#[derive(Debug)]
pub enum CanvaToVertBuffErr {
    VertexBufferCreationError(glium::vertex::BufferCreationError),
    IndexBufferCreationError(glium::index::BufferCreationError),
    CacheError(glium::buffer::CopyError),
}

impl<Drawable: CanvaDrawable> Canva<Drawable> {
    pub fn get_vert_buff<F: Facade>(
        &self,
        facade: &F,
    ) -> Result<(VertexBuffer<Vertex>, IndexBuffer<u16>), CanvaToVertBuffErr> {
        let [x_s, y_s] = [(self.position.0+self.width)*2. -1.,(self.position.1+self.height)*2.-1.];
        let [x_i, y_i] = [self.position.0*2.-1.,self.position.1*2.-1.];

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

        let mut copy_vert_buff = VertexBuffer::empty(facade, 4)
            .map_err(CanvaToVertBuffErr::VertexBufferCreationError)?;
        let mut copy_inde_buff = IndexBuffer::empty(facade, PrimitiveType::TriangleFan, 4)
            .map_err(CanvaToVertBuffErr::IndexBufferCreationError)?;


        vert_buff.copy_to( copy_vert_buff.as_mut_slice())
            .map_err(CanvaToVertBuffErr::CacheError)?;
        ind_buff.copy_to(copy_inde_buff.as_mut_slice())
            .map_err(CanvaToVertBuffErr::CacheError)?;  
        
        *self.vert_buff.borrow_mut() = Some(vert_buff);
        *self.inde_buff.borrow_mut() = Some(ind_buff);
        

        Ok((copy_vert_buff, copy_inde_buff))
    }

}

#[allow(unused)]
pub trait CanvaDrawable 
where Self:Sized{
    fn set_z(&mut self, z: f32);
    fn get_z(&self) -> f32;

    fn to_uniform(&self, target_dimension: (u32, u32)) -> Vec<impl Uniforms>;

    fn is_coord_in_relative(&self, coord: (f32, f32)) -> bool;

    fn update(&mut self, dt: f32,window: &Window) {}

    fn on_click(&mut self,coord:(f32,f32)) {}
    fn on_click_release(&mut self) {}
    fn on_drag(&mut self,old_pos:[f32;2],new_pos:[f32;2]){}
}

impl<E:CanvaDrawable> CanvaDrawable for Canva<E> {
    fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    fn get_z(&self) -> f32 {
        self.z
    }

    fn to_uniform(&self, target_dimension: (u32, u32)) -> Vec<impl Uniforms> {
        let mut uniforms = Vec::with_capacity(self.elements.len());
        for elem in self.elements.iter(){
            uniforms.extend(elem.to_uniform(target_dimension));
        }
        uniforms
    }

    fn is_coord_in_relative(&self, coord: (f32, f32)) -> bool {
        let (c_x,c_y) = coord;

        let [x_s, y_s] = [((self.position.0+self.width)),(self.position.1+self.height)];
        let [x_i, y_i] = [self.position.0,self.position.1];

        x_i < c_x && c_x < x_s 
        && y_i < c_y && c_y < y_s
    }

    fn update(&mut self, dt: f32, window: &Window) {
        for elem in self.elements.iter_mut() {
            elem.update(dt,window);
        }
    }

    fn on_click(&mut self,coord: (f32, f32)) {
        for ball in &mut self.elements {
            if ball.is_coord_in_relative(coord) {
                ball.on_click(coord);
            }
        }
    }
}
