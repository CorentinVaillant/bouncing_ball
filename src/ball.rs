// #![allow(dead_code, unused_variables)]

const PHYSIC_SUB_STEP: u16 = 10;

use glium::dynamic_uniform;

use crate::{
    canvas::CanvasData,
    constants::{FRICTION_COEF, GRAVITY_CONST, MOUSE_ACCELERATION_FACTOR},
    traits::CanvasDrawable,
};

pub type Color = [f32; 3];

#[derive(Clone, Copy)]
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
        }
    }
}

impl CanvasDrawable for Ball {
    fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    fn get_z(&self) -> f32 {
        self.z
    }

    fn update(&mut self, canva_info: &CanvasData, dt: f32) {
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

                let (b_x, b_y): (f32, f32) = (
                    canva_info.size.0 * canva_info.window_resolution.0 as f32,
                    canva_info.size.1 * canva_info.window_resolution.1 as f32,
                );

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

            let force_magnitude = (a_x.powi(2) + a_y.powi(2)).sqrt();
            let max_force = 500.0;
            let color_intensity = (force_magnitude / max_force).clamp(0.0, 1.0);
            self.color = [color_intensity, 0.1, 1.0 - color_intensity];
        }
    }

    fn canvas_uniforms(&self) -> Vec<glium::uniforms::DynamicUniforms> {
        vec![dynamic_uniform! {
            position:&self.position,
            radius: &self.size,
            color:&self.color,
            z:&self.z,
        }]
    }

    fn on_click(&mut self, _: (f32, f32)) {
        self.do_physics = false;
        self.color = [0.2, 1., 0.2];
    }

    fn on_click_release(&mut self) {
        self.do_physics = true;
    }

    fn on_drag(&mut self, old_pos: [f32; 2], new_pos: [f32; 2]) {
        self.do_physics = false;
        self.position = new_pos;
        self.speed = [new_pos[0] - old_pos[0], new_pos[1] - old_pos[1]];
        self.speed = [
            self.speed[0] * MOUSE_ACCELERATION_FACTOR,
            self.speed[1] * MOUSE_ACCELERATION_FACTOR,
        ];
    }

    fn is_absolute_coord_in(&self, coord: (f32, f32)) -> bool {
        let distance =
            ((self.position[0] - coord.0).powi(2) + (self.position[1] - coord.1).powi(2)).sqrt();

        distance < self.size
    }
}

//----------
//Physics
//----------

impl Ball {
    pub fn apply_acceleration(&mut self, dt: f32) {
        self.speed[0] += self.acc[0] * dt;
        self.speed[1] += self.acc[1] * dt;
    }

    pub fn handle_gravity_ball(&mut self) {
        self.acc[1] -= GRAVITY_CONST * self.mass;
    }

    pub fn handle_border_colision_ball(&mut self, (b_x, b_y): (f32, f32)) {
        let [x, y] = &mut self.position;
        let [s_x, s_y] = &mut self.speed;
        let size = self.size;
        let bounce = self.bounce;

        // Bounding box
        if *x < size {
            *x = size; // prevent sticking
            *s_x = -*s_x * bounce;
        } else if *x > b_x - size {
            *x = b_x - size; // prevent sticking
            *s_x = -*s_x * bounce;
        }

        if *y < size {
            *y = size; // prevent sticking
            *s_y = -*s_y * bounce;
        } else if *y > b_y - size {
            *y = b_y - size; // prevent sticking
            *s_y = -*s_y * bounce;
        }
    }

    pub fn handle_collision_balls(&mut self, other: &mut Ball) {
        let dx = other.position[0] - self.position[0];
        let dy = other.position[1] - self.position[1];

        let dist = (dx * dx + dy * dy).sqrt();
        let min_dist = self.size + other.size;

        if dist < min_dist {
            let nx = dx / dist;
            let ny = dy / dist;

            let overlap = min_dist - dist;
            let correction_factor = 0.5;

            self.position[0] -= nx * overlap * correction_factor;
            self.position[1] -= ny * overlap * correction_factor;

            other.position[0] += nx * overlap * correction_factor;
            other.position[1] += ny * overlap * correction_factor;

            let vx = self.speed[0] - other.speed[0];
            let vy = self.speed[1] - other.speed[1];

            let vel_along_normal = vx * nx + vy * ny;
            if vel_along_normal > 0. {
                return;
            }

            let restitution = self.bounce.min(other.bounce);

            let inv_mass1 = self.mass.recip();
            let inv_mass2 = other.mass.recip();
            let impulse_scalar = -(1. + restitution) * vel_along_normal / (inv_mass1 + inv_mass2);

            let impulse_x = impulse_scalar * nx;
            let impulse_y = impulse_scalar * ny;

            self.speed[0] += impulse_x * inv_mass1;
            self.speed[1] += impulse_y * inv_mass1;

            other.speed[0] -= impulse_x * inv_mass2;
            other.speed[1] -= impulse_y * inv_mass2;
        }
    }

    pub fn apply_friction(&mut self, dt: f32) {
        self.speed[0] *= 1. - FRICTION_COEF * dt;
        self.speed[1] *= 1. - FRICTION_COEF * dt;
    }
    
    pub fn apply_speed(&mut self, dt: f32) {
        self.position[0] += self.speed[0] * dt;
        self.position[1] += self.speed[1] * dt;
    }
    
}
