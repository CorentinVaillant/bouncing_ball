// #![allow(dead_code, unused_variables)]

const PHYSIC_SUB_STEP: u16 = 10;

use std::cell::BorrowMutError;

use glium::dynamic_uniform;

use crate::{canva::{CanvaData, CanvaRef}, constants::{FRICTION_COEF, GRAVITY_CONST, MOUSE_ACCELERATION_FACTOR}, traits::CanvaDrawable};

pub type Color = [f32; 3];

#[derive(Clone)]
pub struct Ball {
    pub size: f32,
    pub color: Color,

    pub z: f32,

    pub position: [f32; 2],
    pub speed: [f32; 2],
    pub acc: [f32; 2],

    pub do_physics: bool,
    pub mass: f32,
    pub bounce: f32,

    parent : Option<CanvaRef>
}

impl Ball {
    pub fn new(size: f32, pos: [f32; 2]) -> Self {
        Self {
            size,
            color: [1.; 3],

            z: 1.,

            position: pos,
            speed: [100.; 2],
            acc: [0.; 2],

            do_physics: true,
            mass: size / 2.,
            bounce: 0.30,

            parent :None,
        }
    }

    pub fn new_into_canva(size: f32, pos: [f32; 2],canva:CanvaRef)-> Result<(), BorrowMutError>{
        let new = Self::new(size, pos);

        new.set_canva_parent(canva)
    }
}


impl CanvaDrawable for Ball {
    fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    fn get_z(&self) -> f32 {
        self.z
    }

    fn set_canva_parent(mut self, canva:CanvaRef) ->Result<(),BorrowMutError>{
        self.parent = Some(canva.clone());
        canva.try_borrow_mut()?.elements.push(Box::new(self));
        Ok(())
    }

    fn update(&mut self,canva_info:&CanvaData, dt: f32) {
        let [x, y] = &mut self.position;
        let [s_x, s_y] = &mut self.speed;
        let [a_x, a_y] = &mut self.acc;

        if self.do_physics {
            let dt = dt / f32::from(PHYSIC_SUB_STEP);
            for _ in 0..PHYSIC_SUB_STEP {
                // Reset forces
                *a_x = 0.0;
                *a_y = 0.0;

                // Compute forces

                let (b_x, b_y): (f32, f32) = (canva_info.size.0 * canva_info.window_resolution.0 as f32, canva_info.size.1 * canva_info.window_resolution.1 as f32, );

                //Gravity
                *a_y -= GRAVITY_CONST * self.mass;

                // //Spring like bounce
                // if *x < self.size || (*x - b_x).abs() < self.size {
                //     *a_x += -(*x - (b_x / 2.0)) * (self.size / 2.0) * self.bounce;
                // }
                // if *y < self.size || (*y - b_y).abs() < self.size {
                //     *a_y += -(*y - (b_y / 2.0)) * (self.size / 2.0) * self.bounce;
                // }

                // Bounding box
                if *x < self.size {
                    *x = self.size; // prevent sticking
                    *s_x = -*s_x * self.bounce;
                } else if *x > b_x - self.size {
                    *x = b_x - self.size; // prevent sticking
                    *s_x = -*s_x * self.bounce;
                }

                if *y < self.size {
                    *y = self.size; // prevent sticking
                    *s_y = -*s_y * self.bounce;
                } else if *y > b_y - self.size {
                    *y = b_y - self.size; // prevent sticking
                    *s_y = -*s_y * self.bounce;
                }
                

                // Apply acceleration
                *s_x += *a_x * dt;
                *s_y += *a_y * dt;

                // Apply friction
                *s_x *= 1. - FRICTION_COEF * dt;
                *s_y *= 1. - FRICTION_COEF * dt;

                *x += *s_x * dt;
                *y += *s_y * dt;
            }
            
            
            //todo fixme : better color 
            let force_magnitude = (a_x.powi(2) + a_y.powi(2)).sqrt();
            let color_intensity = (force_magnitude / 1000.0).clamp(0.0, 1.0); // Adjust scaling factor as needed
            self.color = [color_intensity, 0.1, 1.0 - color_intensity];
        }
    }
    
    fn canva_uniform(&self)->glium::uniforms::DynamicUniforms {
        dynamic_uniform! {
            position:&self.position,
            radius: &self.size,
            color:&self.color,
            z:&self.z,
        }
    }


    fn on_click(&mut self,_:(f32,f32)) {
        self.do_physics = false;
        self.color = [0.2, 1., 0.2];
    }

    fn on_click_release(&mut self) {
        self.do_physics = true;
    }

    fn on_drag(&mut self,old_pos:[f32;2],new_pos:[f32;2]) {
        self.position = new_pos;
        self.speed = [new_pos[0]-old_pos[0],new_pos[1]-old_pos[1]];
        self.speed = [self.speed[0]*MOUSE_ACCELERATION_FACTOR,self.speed[1]*MOUSE_ACCELERATION_FACTOR];
    }
    

}

// impl CanvaDrawable for Ball {
//     fn set_z(&mut self, z: f32) {
//         self.z = z;
//     }
//     fn get_z(&self) -> f32 {
//         self.z
//     }

//     fn draw(&self) -> DynamicUniforms {

//         dynamic_uniform! {
//             position : &self.position,
//             radius: &self.size,
//             color: &self.color,
//             z:&self.color,
//         }
//     }

    

//     fn is_relative_coord_in(&self, coord: (f32, f32)) -> bool {

//         println!("distance :{}, size :{}",((self.position[0] - coord.0).powi(2) + (self.position[1] - coord.1).powi(2)).sqrt(), self.size);
//         ((self.position[0] - coord.0).powi(2) + (self.position[1] - coord.1).powi(2)).sqrt()
//             < self.size
//     }


// }
