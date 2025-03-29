
use glium::{backend::Facade, uniforms::DynamicUniforms, Frame};
use crate::constants::*;

use crate::{ball::Ball, canvas::CanvasData};

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
pub trait CanvasDrawable 
{
    fn set_z(&mut self, z: f32);
    fn get_z(&self) -> f32;

    fn is_absolute_coord_in(&self, coord: (f32, f32),) -> bool {false}
    fn is_relative_coord_in(&self, coord: (f32, f32),) -> bool {false}

    fn update(&mut self,canva_info:&CanvasData, dt: f32) {}

    fn on_click(&mut self,coord:(f32,f32)) {}
    fn on_click_release(&mut self) {}
    fn on_drag(&mut self,old_pos:[f32;2],new_pos:[f32;2]){}

    fn on_window_moved(&mut self,new_pos :(f32,f32)){}
    fn on_window_resized(&mut self,new_size :(u32,u32)){}

    fn canvas_uniforms(&self)->Vec<DynamicUniforms>;
}
 
impl<T:CanvasDrawable> CanvasDrawable for Vec<T>{
    fn set_z(&mut self, z: f32) {
        for elem in self{
            elem.set_z(z);
        }
    }

    fn get_z(&self) -> f32 {
        self.get(0).map(CanvasDrawable::get_z)
        .unwrap_or(0.)
    }

    fn canvas_uniforms(&self)->Vec<DynamicUniforms> {
        let mut result = Vec::with_capacity(self.len());
        for elem in self{
            let mut uni = elem.canvas_uniforms();
            result.append(&mut uni);
        }
        result
    }

    fn is_absolute_coord_in(&self, coord: (f32, f32),) -> bool {
        for elem in self{
            if elem.is_absolute_coord_in(coord){
                return true;
            }
        }
        false
    }

    fn is_relative_coord_in(&self, coord: (f32, f32),) -> bool {
        for elem in self{
            if elem.is_relative_coord_in(coord){
                return true;
            }
        }
        false
    }

    fn update(&mut self,canva_info:&CanvasData, dt: f32) {
        for elem in self{
            elem.update(canva_info, dt);
        }
    }

    fn on_click(&mut self,coord:(f32,f32)) {
        for elem in self{
            if elem.is_absolute_coord_in(coord){
                elem.on_click(coord);
            }
        }
    }

    fn on_click_release(&mut self) {
        for elem in self{
            elem.on_click_release();
        }
    }

    fn on_drag(&mut self,old_pos:[f32;2],new_pos:[f32;2]) {
        for elem in self{
            if elem.is_absolute_coord_in(old_pos.into()){
                elem.on_drag(old_pos, new_pos);
            }
        }
    }

    fn on_window_moved(&mut self,new_pos :(f32,f32)) {
        for elem in self{
            elem.on_window_moved(new_pos);
        }
    }

    fn on_window_resized(&mut self,new_size :(u32,u32)) {
        for elem in self{
            elem.on_window_resized(new_size);
        }
    }
}

pub struct Balls{
    pub balls : Vec<Ball>,

    pub z:f32,
}

impl CanvasDrawable for Balls {
    fn set_z(&mut self, z: f32) {

        self.z = z;
        for ball in &mut self.balls {
            ball.set_z(self.z);
        }
    }

    fn canvas_uniforms(&self)->Vec<DynamicUniforms> {
        let mut result = Vec::with_capacity(self.balls.len());
        for ball in &self.balls{
            let mut uni = ball.canvas_uniforms();
            result.append(&mut uni);
        }

        result
    }
    
    fn get_z(&self) -> f32 {
        self.z
    }

    fn is_absolute_coord_in(&self, coord: (f32, f32),) -> bool {
        for elem in &self.balls{
            if elem.is_absolute_coord_in(coord){
                return true;
            }
        }
        false
    }

    fn is_relative_coord_in(&self, coord: (f32, f32),) -> bool {
        for elem in &self.balls{
            if elem.is_relative_coord_in(coord){
                return true;
            }
        }
        false
    }

    fn update(&mut self,canva_info:&CanvasData, dt: f32) {
        const PHYSIC_SUB_STEP :u16 = 1;
        let balls = &mut self.balls;

        for i_ball in 0..balls.len(){

            if balls[i_ball].do_physics {
                // Reset forces
                let [mut x,mut y]     =balls[i_ball].position;
                let [mut s_x,mut s_y] =balls[i_ball].speed;
                let mut a_x = 0.0;
                let mut a_y = 0.0;

                let dt = dt / f32::from(PHYSIC_SUB_STEP);
                for _ in 0..PHYSIC_SUB_STEP {

                    // Compute forces

                    let (b_x, b_y): (f32, f32) = (canva_info.size.0 * canva_info.window_resolution.0 as f32, canva_info.size.1 * canva_info.window_resolution.1 as f32, );

                    //Gravity
                    a_y -= GRAVITY_CONST * balls[i_ball].mass;

                    for j_ball in 0..balls.len(){
                        if j_ball != i_ball{
                            //Spring like bounce
                            let other_ball = &balls[j_ball];



                            if ((x - other_ball.position[0]).powi(2) + (y - other_ball.position[1]).powi(2)).sqrt() < balls[i_ball].size{
                                a_x += -(x - (other_ball.position[0] * other_ball.mass)) *  balls[i_ball].bounce /* * (balls[i_ball].size / 2.0) */;
                                a_y += -(y - (other_ball.position[1] * other_ball.mass)) *  balls[i_ball].bounce /* * (balls[i_ball].size / 2.0) */;

                            }
                        }
                    }


                    // Bounding box
                    if x < balls[i_ball].size {
                        x = balls[i_ball].size; // prevent sticking
                        s_x = -s_x * balls[i_ball].bounce;
                    } else if x > b_x - balls[i_ball].size {
                        x = b_x - balls[i_ball].size; // prevent sticking
                        s_x = -s_x * balls[i_ball].bounce;
                    }

                    if y < balls[i_ball].size {
                        y = balls[i_ball].size; // prevent sticking
                        s_y = -s_y * balls[i_ball].bounce;
                    } else if y > b_y - balls[i_ball].size {
                        y = b_y - balls[i_ball].size; // prevent sticking
                        s_y = -s_y * balls[i_ball].bounce;
                    }
                    

                    // Apply acceleration
                    s_x += a_x * dt;
                    s_y += a_y * dt;

                    // Apply friction
                    s_x *= 1. - FRICTION_COEF * dt;
                    s_y *= 1. - FRICTION_COEF * dt;

                    x += s_x * dt;
                    y += s_y * dt;
                }
                
                
                let force_magnitude = (a_x.powi(2) + a_y.powi(2)).sqrt();
                let max_force = 500.0;
                let color_intensity = (force_magnitude / max_force).clamp(0.0, 1.0);
                balls[i_ball].color = [color_intensity, 0.1, 1.0 - color_intensity];

                balls[i_ball].position = [x,y];
                balls[i_ball].speed = [s_x,s_y];
                balls[i_ball].acc = [a_x,a_y];
            }
    
        }
    }


    fn on_click(&mut self,coord:(f32,f32)) {
        for elem in &mut self.balls{
            if elem.is_absolute_coord_in(coord){
                elem.on_click(coord);
            }
        }
    }

    fn on_click_release(&mut self) {
        for elem in &mut self.balls{
            elem.on_click_release();
        }
    }

    fn on_drag(&mut self,old_pos:[f32;2],new_pos:[f32;2]) {
        for elem in &mut self.balls{
            if elem.is_absolute_coord_in(old_pos.into()){
                elem.on_drag(old_pos, new_pos);
            }
        }
    }

    fn on_window_moved(&mut self,new_pos :(f32,f32)) {
        for elem in &mut self.balls{
            elem.on_window_moved(new_pos);
        }
    }

    fn on_window_resized(&mut self,new_size :(u32,u32)) {
        for elem in &mut self.balls{
            elem.on_window_resized(new_size);
        }
    }
}