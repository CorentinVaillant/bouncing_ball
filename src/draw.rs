use glium::{backend::Facade, Frame};

pub trait Drawable {
    type DrawResult;

    fn draw<F: Facade + Sized>(&self, display :&F,frame:&mut Frame)->Self::DrawResult;
}