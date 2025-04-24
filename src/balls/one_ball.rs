use glium::dynamic_uniform;
use my_glium_util::{
    canvas::{CanvasData, traits::CanvasDrawable},
    datastruct::points::As2dPoint,
    math::{EuclidianSpace, Vec2, VectorSpace},
};

use crate::physics::{
    constants::{FRICTION_COEF, GRAVITY_CONST, LIGHT_SPEED, MOUSE_ACCELERATION_FACTOR},
    traits::Physics,
};

pub type Color = [f32; 3];

#[derive(Clone, Copy)]
pub struct Ball {
    pub size: f32,
    pub color: Color,

    pub z: f32,
    pub id: usize,

    pub coliding_pos: Vec2,
    pub nb_coll: usize,

    pub position: Vec2,
    pub speed: Vec2,
    pub acc: Vec2,

    pub do_physics: bool,
    pub mass: f32,
    pub bounce: f32,

    canva_info: Option<CanvasData>,
}

impl Ball {
    pub fn new(size: f32, pos: [f32; 2], id: usize) -> Self {
        debug_assert!(!(pos[0].is_nan() || pos[1].is_nan()));

        Self {
            size,
            color: [1.; 3],

            z: 1.,
            id,

            coliding_pos: pos.into(),
            nb_coll: 0,

            position: pos.into(),
            speed: [0.; 2].into(),
            acc: [0.; 2].into(),

            do_physics: true,
            mass: size * size,
            bounce: 0.3,

            canva_info: None,
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
        self.canva_info = Some(*canva_info);
        self.physics_update(dt);
    }
    fn canvas_uniforms(&self) -> Vec<glium::uniforms::DynamicUniforms> {
        vec![dynamic_uniform! {
            position:self.position.as_array(),
            speed: self.speed.as_array(),
            radius: &self.size,
            color:&self.color,
            collision_pos:self.coliding_pos.as_array(),

            z:&self.z,
        }]
    }

    fn on_click(&mut self, _: (f32, f32)) {
        self.do_physics = false;
        self.color = [0.2, 1., 0.2];

        self.speed = Vec2::v_space_zero();
        self.acc = Vec2::v_space_zero();
    }

    fn on_click_release(&mut self) {
        self.do_physics = true;
    }

    fn on_drag(&mut self, old_pos: [f32; 2], new_pos: [f32; 2]) {
        self.coliding_pos = self.position;
        self.do_physics = false;
        self.position = new_pos.into();
        self.speed = [new_pos[0] - old_pos[0], new_pos[1] - old_pos[1]].into();
        self.speed *= MOUSE_ACCELERATION_FACTOR;
    }

    fn is_absolute_coord_in(&self, coord: (f32, f32)) -> bool {
        let distance =
            ((self.position[0] - coord.0).powi(2) + (self.position[1] - coord.1).powi(2)).sqrt();

        distance < self.size
    }
}

//-----------
//| Physics |
//-----------

impl Physics for Ball {
    fn physics_update(&mut self, dt: f32) {
        if !self.do_physics {
            return;
        }

        const PHYSIC_SUB_STEP: u16 = 20;
        let sub_dt = dt / f32::from(PHYSIC_SUB_STEP);

        let border: (f32, f32) = (
            self.canva_info.map(|c| c.size.0).unwrap_or(0.)
                * self
                    .canva_info
                    .map(|c| c.window_resolution.0 as f32)
                    .unwrap_or(0.),
            self.canva_info.map(|c| c.size.1).unwrap_or(0.)
                * self
                    .canva_info
                    .map(|c| c.window_resolution.1 as f32)
                    .unwrap_or(0.),
        );

        for _ in 0..PHYSIC_SUB_STEP {
            self.reset_force();
            self.handle_gravity();
            self.handle_border_colision_ball(border);

            self.apply_acceleration(sub_dt);
            self.handle_friction();
            self.apply_speed(sub_dt);
            self.handle_color();
        }
    }
}

impl Ball {
    const PHYS_MIN_DIST: f32 = 0.001;
    const MAX_VEL: f32 = LIGHT_SPEED / 1_000.;

    pub fn handle_border_colision_ball(&mut self, (b_x, b_y): (f32, f32)) {
        let [x, y] = &mut self.position.as_mut_array();
        let [s_x, s_y] = &mut self.speed.as_mut_array();
        let size = self.size;
        let bounce = self.bounce;

        // Bounding box
        if *x < size {
            *x = size; // prevent sticking
            *s_x *= -bounce;
        } else if *x > b_x - size {
            *x = b_x - size; // prevent sticking
            *s_x *= -bounce;
        }

        if *y < size {
            *y = size; // prevent sticking
            *s_y *= -bounce;
        } else if *y > b_y - size {
            *y = b_y - size; // prevent sticking
            *s_y *= -bounce;
        }
    }

    pub fn is_overlapping(&self, other: &Self) -> bool {
        (self.position[0] - other.position[0]) * (self.position[0] - other.position[0])
            + (self.position[1] - other.position[1]) * (self.position[1] - other.position[1])
            < (self.size + other.size) * (self.size + other.size)
    }

    fn handle_static_collision(&mut self, other: &mut Self) {
        let dist = self
            .position
            .distance(other.position)
            .max(Self::PHYS_MIN_DIST);

        //1. compute distance and overlap factor between balls

        let overlap = 0.5 * ((self.size + other.size) - dist).max(0.0);

        //2. resolve overlap
        self.position += (self.position - other.position) * overlap / dist;
        other.position -= (self.position - other.position) * overlap / dist;
    }

    #[allow(dead_code)]
    fn handle_dynamic_collision_inelastic(&mut self, other: &mut Self) {
        const RESTITUTION_THRESHOLD: f32 = 0.5;

        let bounce = (self.bounce + other.bounce) / 2.;

        // 1. inelastic collision :

        let dist = self
            .position
            .distance(other.position)
            .max(Self::PHYS_MIN_DIST);

        let rel_speed = self.speed - other.speed;
        let norm_p = (self.position - other.position) / dist;

        let rel_vel_along_normal = rel_speed.dot(norm_p);

        if rel_vel_along_normal > RESTITUTION_THRESHOLD {
            return;
        }

        let norm_p: Vec2 = (self.position - other.position) / dist;

        let norm_impulse = rel_vel_along_normal
            * ((self.mass * other.mass) / (self.mass + other.mass))
            * (1.0 + bounce);

        self.speed  -= norm_p * (norm_impulse / self.mass.max(f32::EPSILON));
        other.speed += norm_p * (norm_impulse / other.mass.max(f32::EPSILON));
    }

    #[allow(dead_code)]
    fn handle_dynamic_collision_elastic(&mut self, other: &mut Self) {
        let norm_p = (self.position - other.position)
            / self
                .position
                .distance(other.position)
                .max(Self::PHYS_MIN_DIST);
        let tan_p: Vec2 = [-norm_p[0], norm_p[1]].into();

        let dotprod_tan_self = self.speed.dot(tan_p);
        let dotprod_tan_other = other.speed.dot(tan_p);
        let dotprod_norm_self = self.speed.dot(norm_p);
        let dotprod_norm_other = other.speed.dot(norm_p);

        let m_self = (dotprod_norm_self * (self.mass - other.mass)
            + 2. * other.mass * dotprod_norm_other)
            / (self.mass + other.mass);
        let m_other = (dotprod_norm_other * (other.mass - self.mass)
            + 2. * self.mass * dotprod_norm_self)
            / (self.mass + other.mass);

        self.speed = tan_p * dotprod_tan_self + norm_p * m_self;
        other.speed = tan_p * dotprod_tan_other + norm_p * m_other;
    }

    ///Handle collision between two balls.
    /// this video has been very usefull to make the physics behind this :  
    ///     -> https://www.youtube.com/watch?v=LPzyNOHY3A4
    pub fn handle_collision_balls(&mut self, other: &mut Ball, _dt: f32) {
        if self.is_overlapping(other) && self.nb_coll < 100 {
            self.coliding_pos = other.position;
            self.nb_coll += 1;

            //I static collision :
            self.handle_static_collision(other);

            //II dynamic response using impulse (see: https://en.wikipedia.org/wiki/Inelastic_collision)
            // self.handle_dynamic_collision_elastic(other);
            self.handle_dynamic_collision_inelastic(other);
        }
    }

    pub fn reset_force(&mut self) {
        self.nb_coll = 0;
        self.coliding_pos = self.position;
        self.acc = Vec2::v_space_zero();
    }

    pub fn apply_acceleration(&mut self, dt: f32) {
        self.speed[0] += self.acc[0] * dt;
        self.speed[1] += self.acc[1] * dt;

    }

    pub fn handle_gravity(&mut self) {
        self.acc[1] += GRAVITY_CONST * self.mass;
    }

    pub fn handle_friction(&mut self) {
        self.acc -= self.speed * FRICTION_COEF;
    }

    pub fn apply_speed(&mut self, dt: f32) {
        self.speed[0] = self.speed[0].clamp(-Self::MAX_VEL, Self::MAX_VEL);
        self.speed[1] = self.speed[1].clamp(-Self::MAX_VEL, Self::MAX_VEL);

        self.position[0] += self.speed[0] * dt;
        self.position[1] += self.speed[1] * dt;
    }

    pub fn handle_color(&mut self) {
        // let f = f32::sin(self.id as f32 * f32::consts::FRAC_PI_2 ).abs();
        self.color = hue_to_rgb(self.id as f32 * 4. * std::f32::consts::FRAC_PI_2 / 32.);
    }
}

impl As2dPoint<f32> for Ball {
    #[inline]
    fn x(&self) -> f32 {
        self.position[0]
    }

    #[inline]
    fn y(&self) -> f32 {
        self.position[1]
    }
}

fn hue_to_rgb(h: f32) -> [f32; 3] {
    let h = h % (2. * std::f32::consts::PI);
    let c = 1.0;
    let h_prime = h / (std::f32::consts::FRAC_PI_3);
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());

    match h_prime as u32 {
        0 => [c, x, 0.0],
        1 => [x, c, 0.0],
        2 => [0.0, c, x],
        3 => [0.0, x, c],
        4 => [x, 0.0, c],
        5 => [c, 0.0, x],
        _ => [1.0, 0.0, 0.0], // fallback (shouldn't happen)
    }
}
