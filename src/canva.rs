#![allow(dead_code)]


use std::{cell::{BorrowMutError, RefCell}, rc::Rc};

use glium::{
    backend::Facade, dynamic_uniform, index::PrimitiveType, DepthTest, DrawParameters, Frame, IndexBuffer, Program, Surface, VertexBuffer
};

use crate::{traits::{self, CanvaDrawable, Drawable}, vertex::Vertex};

pub type CanvaRef = Rc<RefCell<Canva>>;

pub struct Canva {

    pub data:CanvaData,
    pub elements: Vec<Box<dyn CanvaDrawable>>,

    program: Program,

    
    z : f32,

    parent: Option<CanvaRef>,
}

#[derive(Debug,Clone, Copy)]
pub struct CanvaData{
    pub size: (f32,f32),
    pub position : (f32,f32), //%

    pub window_resolution:(u32,u32),


}

impl Canva{
    pub fn new(position:(f32,f32) ,program: Program) -> Self {
        Self {
            data :CanvaData{
                position,
                size : (1.,1.),

                window_resolution:(0,0),
            },
            elements:vec![],
            program,

            z:0.,

            parent:None,
        }
    }
}

impl Drawable for Canva{
    fn draw<F: Facade + Sized>(&self, facade: &F, target: &mut Frame) -> Result<(), traits::DrawError> {
        let (vert_buff, ind_buff) = self.get_vert_buff(facade).unwrap();

        let mut param = DrawParameters::default();
        param.depth.test = DepthTest::IfMore;


        for elem in &self.elements{
            let mut uniform = elem.canva_uniform();
    
            
            uniform.add("canva_z",&self.z,);
            uniform.add("canva_pos",&self.data.position,);
            uniform.add("canva_size",&self.data.size);
            
            
            target
                .draw(&vert_buff, &ind_buff, &self.program, &uniform, &param)
                .unwrap();
        };

        Ok(())
    }
}

impl CanvaDrawable for Canva{
    fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    fn get_z(&self) -> f32 {
        self.z
    }

    fn canva_uniform(&self)->glium::uniforms::DynamicUniforms {
        dynamic_uniform! {
            canva_z : &self.z,
            canva_pos:&self.data.position,
            canva_size:&self.data.size,
        }
    }
    
    fn set_canva_parent(mut self, canva:CanvaRef) ->Result<(),BorrowMutError>{
        self.parent = Some(canva.clone());
        canva.try_borrow_mut()?.elements.push(Box::new(self));
        Ok(())
    }

    fn update(&mut self,_canva_info:&CanvaData ,dt: f32) {
        for elem in &mut self.elements{
            elem.update(&self.data,dt);
        }
    }

    fn window_resized(&mut self,new_size :(u32,u32)) {
        self.data.window_resolution=new_size;
        for elem in &mut self.elements{
            elem.window_resized(new_size);
        }
    }

    fn on_drag(&mut self,old_pos:[f32;2],new_pos:[f32;2]) {
        println!("draging from {old_pos:?} to {new_pos:?}");
    }


    //TODO fix coord in 
    fn is_absolute_coord_in(&self, coord: (f32, f32),) -> bool {
        
        let h_dist = coord.0 - self.data.position.0 + self.data.size.0 * self.data.window_resolution.0 as f32;
        let v_dist = coord.1 - self.data.position.1 + self.data.size.1 * self.data.window_resolution.1 as f32;

        let dist = (v_dist.powi(2)+h_dist.powi(2)).sqrt();

        println!("click distance :{dist}");

        0. <= coord.0 && coord.0 <= self.data.position.0 + self.data.size.0 * self.data.window_resolution.0 as f32 &&
        0. <= coord.1 && coord.1 <= self.data.position.0 + self.data.size.1 * self.data.window_resolution.1 as f32 
    }

    fn is_relative_coord_in(&self, coord: (f32, f32),) -> bool {
        
        let h_dist = coord.0 - self.data.position.0 + self.data.size.0 ;
        let v_dist = coord.1 - self.data.position.1 + self.data.size.1 ;

        let dist = (v_dist.powi(2)+h_dist.powi(2)).sqrt();

        println!("click distance :{dist}");

        0. <= coord.0 && coord.0 <= self.data.position.0 + self.data.size.0  &&
        0. <= coord.1 && coord.1 <= self.data.position.0 + self.data.size.1  
    }
}

#[derive(Debug)]
pub enum CanvaToVertBuffErr {
    VertexBufferCreationError(glium::vertex::BufferCreationError),
    IndexBufferCreationError(glium::index::BufferCreationError),
    CacheError(glium::buffer::CopyError),
}

impl Canva {
    pub fn get_vert_buff<F: Facade>(
        &self,
        facade: &F,
    ) -> Result<(VertexBuffer<Vertex>, IndexBuffer<u16>), CanvaToVertBuffErr> {
        let [x_s, y_s] = [(self.data.position.0+self.data.size.0)*2. -1.,(self.data.position.1+self.data.size.1)*2.-1.];
        let [x_i, y_i] = [self.data.position.0*2.-1.,self.data.position.1*2.-1.];

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
