use glium::uniforms::DynamicUniforms;

use crate::{
    ball::Ball,
    canvas::CanvasData,
    traits::CanvasDrawable,
};

pub struct Balls {
    pub balls: Vec<Ball>,

    pub z: f32,
}

impl CanvasDrawable for Balls {
    fn set_z(&mut self, z: f32) {
        self.z = z;
        for ball in &mut self.balls {
            ball.set_z(self.z);
        }
    }

    fn canvas_uniforms(&self) -> Vec<DynamicUniforms> {
        let mut result = Vec::with_capacity(self.balls.len());
        for ball in &self.balls {
            let mut uni = ball.canvas_uniforms();
            result.append(&mut uni);
        }

        result
    }

    fn get_z(&self) -> f32 {
        self.z
    }

    fn is_absolute_coord_in(&self, coord: (f32, f32)) -> bool {
        for elem in &self.balls {
            if elem.is_absolute_coord_in(coord) {
                return true;
            }
        }
        false
    }

    fn is_relative_coord_in(&self, coord: (f32, f32)) -> bool {
        for elem in &self.balls {
            if elem.is_relative_coord_in(coord) {
                return true;
            }
        }
        false
    }

    fn update(&mut self, canva_info: &CanvasData, dt: f32) {
        const PHYSIC_SUB_STEP: u16 = 1;
        let balls = &mut self.balls;

        for i_ball in 0..balls.len() {
            if balls[i_ball].do_physics {
                // Reset forces

                let mut ball = balls[i_ball].clone();
                // Compute forces
                ball.acc = [0.; 2];

                let dt = dt / f32::from(PHYSIC_SUB_STEP);
                for _ in 0..PHYSIC_SUB_STEP {
                    let (b_x, b_y): (f32, f32) = (
                        canva_info.size.0 * canva_info.window_resolution.0 as f32,
                        canva_info.size.1 * canva_info.window_resolution.1 as f32,
                    );
                    ball.handle_gravity_ball();
                    ball.handle_border_colision_ball((b_x, b_y));

                    for j_ball in 0..balls.len() {
                        if j_ball != i_ball {
                            if ball.position == balls[j_ball].position {
                                println!("hmmm");
                            }
                            ball.handle_collision_balls(&mut balls[j_ball]);
                        }
                    }
                    ball.apply_acceleration(dt);
                    ball.apply_friction(dt);
                    ball.apply_speed(dt);
                }

                let force_magnitude = (ball.acc[0].powi(2) + ball.acc[1].powi(2)).sqrt();
                let max_force = 500.0;
                let color_intensity = (force_magnitude / max_force).clamp(0.0, 1.0);
                balls[i_ball].color = [color_intensity, 0.1, 1.0 - color_intensity];

                balls[i_ball] = ball;
            }
        }
    }

    fn on_click(&mut self, coord: (f32, f32)) {
        for elem in &mut self.balls {
            if elem.is_absolute_coord_in(coord) {
                elem.on_click(coord);
            }
        }
    }

    fn on_click_release(&mut self) {
        for elem in &mut self.balls {
            elem.on_click_release();
        }
    }

    fn on_drag(&mut self, old_pos: [f32; 2], new_pos: [f32; 2]) {
        for elem in &mut self.balls {
            if elem.is_absolute_coord_in(old_pos.into()) {
                elem.on_drag(old_pos, new_pos);
            }
        }
    }

    fn on_window_moved(&mut self, new_pos: (f32, f32)) {
        for elem in &mut self.balls {
            elem.on_window_moved(new_pos);
        }
    }

    fn on_window_resized(&mut self, new_size: (u32, u32)) {
        for elem in &mut self.balls {
            elem.on_window_resized(new_size);
        }
    }
}

