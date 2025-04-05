#![allow(dead_code)]

use glium::uniforms::DynamicUniforms;

use crate::{
    canvas::CanvasData,
    one_ball::Ball,
    quadtree::{AABB, Quadtree},
    traits::CanvasDrawable,
};

pub struct Balls {
    pub boundary: AABB,
    pub balls: Quadtree<Ball, 5>,

    pub z: f32,
}

impl Balls {
    pub fn empty(boundary: AABB) -> Self {
        Self {
            boundary,
            balls: Quadtree::empty(boundary),
            z: 0.,
        }
    }

    pub fn new(boundary: AABB, balls: Vec<Ball>) -> Self {
        let mut qtree = Quadtree::empty(boundary);
        for mut b in balls {
            if qtree.insert(b).is_err() {
                b.handle_border_colision_ball((
                    boundary.center.x + boundary.half_dim,
                    boundary.center.y + boundary.half_dim,
                ));
                println!("replacing ball into :{:?}", b.position);
                let _ = qtree
                    .insert(b)
                    // .expect("could not replace the ball inside qtree::new");
                    .map_err(|e| println!("error : {e:?}, giving up the ball !"));
            }
        }

        Self {
            boundary,
            balls: qtree,
            z: 0.,
        }
    }

    pub fn push_ball(&mut self, ball: Ball) {
        self.balls.insert_fit(ball);
    }
}

impl CanvasDrawable for Balls {
    fn set_z(&mut self, z: f32) {
        self.z = z;
        self.balls.map_query_range(self.boundary, |b| {
            b.set_z(z);
        });
    }

    fn canvas_uniforms(&self) -> Vec<DynamicUniforms> {
        let mut result = Vec::with_capacity(self.balls.len());
        for ball in self.balls.query_range(self.boundary) {
            let mut uni = ball.canvas_uniforms();
            result.append(&mut uni);
        }

        result
    }

    fn get_z(&self) -> f32 {
        self.z
    }

    fn is_absolute_coord_in(&self, _coord: (f32, f32)) -> bool {
        true
        // for elem in &self.balls {
        //     if elem.is_absolute_coord_in(coord) {
        //         return true;
        //     }
        // }
        // false
    }

    fn is_relative_coord_in(&self, _coord: (f32, f32)) -> bool {
        true
        // for elem in &self.balls {
        //     if elem.is_relative_coord_in(coord) {
        //         return true;
        //     }
        // }
        // false
    }

    fn update(&mut self, canva_info: &CanvasData, dt: f32) {
        const PHYSIC_SUB_STEP: u16 = 10;
        let sub_dt = dt / f32::from(PHYSIC_SUB_STEP);
        let balls = &mut self.balls;

        let border: (f32, f32) = (
            (self.boundary.center.x + self.boundary.half_dim)
                .min(canva_info.size.0 * canva_info.window_resolution.0 as f32),
            (self.boundary.center.y + self.boundary.half_dim)
                .min(canva_info.size.1 * canva_info.window_resolution.1 as f32),
        );

        let range_map = |ball: &Ball| AABB::new(ball.position.into(), ball.size + 1.);

        let update_ball = |ball: &mut Ball, other_ball: &mut Ball| {
            ball.handle_collision_balls(other_ball, sub_dt);
        };

        for _ in 0..PHYSIC_SUB_STEP {

            balls.iter_mut().for_each(|ball|{
                if !ball.do_physics{return;}
                ball.reset_force();
                ball.handle_gravity();
                ball.handle_border_colision_ball(border);
            });
            balls.map_with_elem_in_range(range_map, update_ball);
            // println!("3. \t=> making the rest of the physics"); 

            balls.iter_mut().for_each(|ball|{
                if !ball.do_physics{return;}
                ball.apply_acceleration(sub_dt);
                ball.apply_friction(sub_dt);
                ball.apply_speed(sub_dt);
            });

        }

        balls.iter_mut().for_each(|ball|{
            if ball.do_physics{
            ball.handle_color();}
        });

        balls.rebuild();
    }

    fn on_click(&mut self, coord: (f32, f32)) {
        let mut clicking_on_ball = false;
        for elem in &mut self.balls.iter_mut() {
            if elem.is_absolute_coord_in(coord) {
                elem.on_click(coord);
                clicking_on_ball = true;
                println!("clicking on ball");
            }
        }
        if !clicking_on_ball {
            println!("adding ball at :{coord:?}");
            self.push_ball(Ball::new(20., coord.into()))
        }
    }

    fn on_click_release(&mut self) {
        for elem in self.balls.iter_mut() {
            elem.on_click_release();
        }
    }

    fn on_drag(&mut self, old_pos: [f32; 2], new_pos: [f32; 2]) {
        for elem in self.balls.iter_mut() {
            if elem.is_absolute_coord_in(old_pos.into()) {
                elem.on_drag(old_pos, new_pos);
            }
        }
    }

    fn on_window_moved(&mut self, new_pos: (f32, f32)) {
        for elem in self.balls.iter_mut() {
            elem.on_window_moved(new_pos);
        }
    }

    fn on_window_resized(&mut self, new_size: (u32, u32)) {
        for elem in self.balls.iter_mut() {
            elem.on_window_resized(new_size);
        }
    }
}
