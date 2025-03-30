use glium::{Frame, backend::Facade, uniforms::DynamicUniforms};

use crate::canvas::CanvasData;

/********************\
|-------Drawable-----|
\********************/

#[derive(Debug)]
pub enum DrawError {}

pub trait Drawable {
    fn draw<F: Facade + Sized>(&self, display: &F, frame: &mut Frame) -> Result<(), DrawError>;
}

/*******************************************************************************************/

/*********************\
|----CanvaDrawable----|
\*********************/

#[allow(unused)]
pub trait CanvasDrawable {
    fn set_z(&mut self, z: f32);
    fn get_z(&self) -> f32;

    fn is_absolute_coord_in(&self, coord: (f32, f32)) -> bool {
        false
    }
    fn is_relative_coord_in(&self, coord: (f32, f32)) -> bool {
        false
    }

    fn update(&mut self, canva_info: &CanvasData, dt: f32) {}

    fn on_click(&mut self, coord: (f32, f32)) {}
    fn on_click_release(&mut self) {}
    fn on_drag(&mut self, old_pos: [f32; 2], new_pos: [f32; 2]) {}

    fn on_window_moved(&mut self, new_pos: (f32, f32)) {}
    fn on_window_resized(&mut self, new_size: (u32, u32)) {}

    fn canvas_uniforms(&self) -> Vec<DynamicUniforms>;
}

impl<T: CanvasDrawable> CanvasDrawable for Vec<T> {
    fn set_z(&mut self, z: f32) {
        for elem in self {
            elem.set_z(z);
        }
    }

    fn get_z(&self) -> f32 {
        self.get(0).map(CanvasDrawable::get_z).unwrap_or(0.)
    }

    fn canvas_uniforms(&self) -> Vec<DynamicUniforms> {
        let mut result = Vec::with_capacity(self.len());
        for elem in self {
            let mut uni = elem.canvas_uniforms();
            result.append(&mut uni);
        }
        result
    }

    fn is_absolute_coord_in(&self, coord: (f32, f32)) -> bool {
        for elem in self {
            if elem.is_absolute_coord_in(coord) {
                return true;
            }
        }
        false
    }

    fn is_relative_coord_in(&self, coord: (f32, f32)) -> bool {
        for elem in self {
            if elem.is_relative_coord_in(coord) {
                return true;
            }
        }
        false
    }

    fn update(&mut self, canva_info: &CanvasData, dt: f32) {
        for elem in self {
            elem.update(canva_info, dt);
        }
    }

    fn on_click(&mut self, coord: (f32, f32)) {
        for elem in self {
            if elem.is_absolute_coord_in(coord) {
                elem.on_click(coord);
            }
        }
    }

    fn on_click_release(&mut self) {
        for elem in self {
            elem.on_click_release();
        }
    }

    fn on_drag(&mut self, old_pos: [f32; 2], new_pos: [f32; 2]) {
        for elem in self {
            if elem.is_absolute_coord_in(old_pos.into()) {
                elem.on_drag(old_pos, new_pos);
            }
        }
    }

    fn on_window_moved(&mut self, new_pos: (f32, f32)) {
        for elem in self {
            elem.on_window_moved(new_pos);
        }
    }

    fn on_window_resized(&mut self, new_size: (u32, u32)) {
        for elem in self {
            elem.on_window_resized(new_size);
        }
    }
}
