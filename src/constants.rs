use my_rust_matrix_lib::my_matrix_lib::prelude::VectorMath;

pub const FRICTION_COEF: f32 = 0.8;
pub const GRAVITY_CONST: f32 = 0.;//9.80665;
pub const MOUSE_ACCELERATION_FACTOR: f32 = 20.;
pub const LIGHT_SPEED: f32 = 299_792_458.;

pub type Vec2 = VectorMath<f32, 2>;
