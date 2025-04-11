//! Définition of a physics trait, that represent all object that have physics simulation

pub trait Physics {
    // fn get_acc(&mut self)->&mut Vec2;
    // fn get_vel(&mut self)->&mut Vec2;
    // fn get_pos(&mut self)->&mut Vec2;

    fn physics_update(&mut self, dt: f32);
}
