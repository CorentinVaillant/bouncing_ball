#![cfg(test)]

// mod fix_physics_test {
//     use std::default;

//     use super::{
//         constants::Vec2,
//         fix_physics::{PhysicsFixUpdates, PhysicsWrapper},
//         traits::Physics,
//     };

//     const TEST_PHYSIC_STRUCT_ONE_MASS: f32 = 10.;
//     struct TestPhysicStructOne {
//         position: Vec2,
//     }
//     impl Default for TestPhysicStructOne {
//         fn default() -> Self {
//             Self {
//                 position: Default::default(),
//             }
//         }
//     }

//     struct TestPhysicStructTwo {
//         position: Vec2,
//     }

//     impl Default for TestPhysicStructTwo {
//         fn default() -> Self {
//             Self {
//                 position: Default::default(),
//             }
//         }
//     }

//     impl Physics for TestPhysicStructOne {
//         fn physics_update(&mut self, dt: f32) {
//             self.position -= Vec2::from([0., 1.]) * TEST_PHYSIC_STRUCT_ONE_MASS * dt * dt;
//         }
//     }

//     impl Physics for TestPhysicStructTwo {
//         fn physics_update(&mut self, dt: f32) {
//             self.position -= Vec2::from([f32::sin(dt), f32::cos(dt)]);
//         }
//     }

//     #[test]
//     fn test_fix_physic_update() {
//         let mut wrapper = PhysicsWrapper::empty();
//         let struct_1 = TestPhysicStructOne::default();
//         let struct_2 = TestPhysicStructTwo::default();

//         wrapper.push(struct_1);
//         wrapper.push(struct_2);

//         let fix_phy = PhysicsFixUpdates::new(wrapper, 60_f32.recip());
//     }
// }
