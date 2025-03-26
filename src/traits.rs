use std::cell::BorrowMutError;

use glium::{backend::Facade, uniforms::DynamicUniforms, Frame};

use crate::canva::{CanvaData, CanvaRef};

/********************\
|-------Drawable-----|
\********************/

#[derive(Debug)]
pub enum DrawError{

    
}

pub trait Drawable {

    fn draw<F: Facade + Sized>(&self, display :&F,frame:&mut Frame)->Result<(),DrawError>;
}

/*******************************************************************************************/

/*********************\
|----CanvaDrawable----|
\*********************/

#[allow(unused)]
pub trait CanvaDrawable 
{
    fn set_z(&mut self, z: f32);
    fn get_z(&self) -> f32;

    fn set_canva_parent(self, canva:CanvaRef)->Result<(),BorrowMutError>;

    fn is_absolute_coord_in(&self, coord: (f32, f32),) -> bool {false}
    fn is_relative_coord_in(&self, coord: (f32, f32),) -> bool {false}

    fn update(&mut self,canva_info:&CanvaData, dt: f32) {}

    fn on_click(&mut self,coord:(f32,f32)) {}
    fn on_click_release(&mut self) {}
    fn on_drag(&mut self,old_pos:[f32;2],new_pos:[f32;2]){}

    fn window_moved(&mut self,new_pos :(f32,f32)){}
    fn window_resized(&mut self,new_size :(u32,u32)){}

    fn canva_uniform(&self)->DynamicUniforms;
}
 
