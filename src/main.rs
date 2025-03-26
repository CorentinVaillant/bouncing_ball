use core::f32;
use std::{cell::RefCell, rc::Rc};

use ball::Ball;
use canva::{Canva, CanvaData, CanvaRef};
use traits::{CanvaDrawable, Drawable};
use glium::{
    Display, Program, Surface, backend,
    glutin::surface::WindowSurface,
    winit::{
        application::ApplicationHandler,
        event::{DeviceEvent, ElementState, MouseButton, WindowEvent},
        event_loop::{self, EventLoop},
        window::Window,
    },
};

mod ball;
mod canva;
mod constants;
mod vertex;
mod traits;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let (window, display) = backend::glutin::SimpleWindowBuilder::new()
        .with_title("Bouncing ball !")
        .build(&event_loop);

    let frag_shad = std::fs::read_to_string("./shaders/ball.frag").unwrap();
    let vert_shad = std::fs::read_to_string("./shaders/ball.vert").unwrap();
    let program = Program::from_source(&display, &vert_shad, &frag_shad, None).unwrap();
    
    let canva = Rc::new(RefCell::new(Canva::new((0.,0.), program)));

    Ball::new_into_canva(100., [200.;2], canva.clone()).unwrap();
    Ball::new_into_canva(25., [100.;2], canva.clone()).unwrap();
    
    let mut app = App {
        main_canva: canva,

        dt: 0.,
        time: std::time::Instant::now(),

        display,
        _window: window,

        mouse_position: (0., 0.),
        mouse_cliking: false,
    };

    event_loop.set_control_flow(event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
}

struct App {
    main_canva: CanvaRef,

    dt: f32,
    time: std::time::Instant,

    display: Display<WindowSurface>,
    _window: Window,

    mouse_position: (f32, f32),
    mouse_cliking: bool,
}

impl App {
    fn draw(&mut self) {
        let mut target = self.display.draw();

        target.clear_color(0.03,0.03,0.03, 1.);
        self.main_canva.borrow().draw(&self.display, &mut target).unwrap();

        target.finish().unwrap()
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &event_loop::ActiveEventLoop) {
        println!("resumed !");
    }

    fn window_event(
        &mut self,
        event_loop: &event_loop::ActiveEventLoop,
        _window_id: glium::winit::window::WindowId,
        event: glium::winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => match event.physical_key {
                glium::winit::keyboard::PhysicalKey::Code(key_code) => match key_code {
                    glium::winit::keyboard::KeyCode::Escape => event_loop.exit(),
                    _ => (),
                },
                glium::winit::keyboard::PhysicalKey::Unidentified(_) => (),
            },
            
            WindowEvent::Resized(new_size) => {
                self.display.resize(new_size.into());
                self.main_canva.borrow_mut().window_resized(new_size.into());
            }
            WindowEvent::Moved(pos) => {
                self.main_canva.borrow_mut().window_moved(pos.into());
            }

            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {

                let new_pos = position.into();
                //draging
                if self.mouse_cliking{
                    if self.main_canva.borrow().is_relative_coord_in(self.mouse_position){
                        self.main_canva.borrow_mut().on_drag(self.mouse_position.into(), new_pos);
                    }
                }
                
                self.mouse_position = new_pos.into();
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => match (button, state) {
                (MouseButton::Left, ElementState::Pressed) => { 
                    self.mouse_cliking = true;
                    if self.main_canva.borrow().is_relative_coord_in(self.mouse_position){
                        self.main_canva.borrow_mut().on_click(self.mouse_position);
                    }
                },
                (MouseButton::Left, ElementState::Released) => {
                    self.mouse_cliking = false;
                    self.main_canva.borrow_mut().on_click_release();
                },
                _ => (),
            },

            _ => (),
        };
    }

    fn device_event(
        &mut self,
        _event_loop: &event_loop::ActiveEventLoop,
        _device_id: glium::winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            _ => (),
        }
    }

    fn exiting(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        println!("exiting...");
        event_loop.exit();
    }

    fn new_events(
        &mut self,
        _event_loop: &event_loop::ActiveEventLoop,
        cause: glium::winit::event::StartCause,
    ) {
        match cause {
            glium::winit::event::StartCause::Init => (),
            glium::winit::event::StartCause::Poll => {
                //dt handling
                let now = std::time::Instant::now();
                self.dt = now.duration_since(self.time).as_secs_f32();
                self.time = now;

                //ball
                self.main_canva.borrow_mut().update(&DUMMY_CANVA_INFO,self.dt);

                //draw
                self.draw();
            }
            _ => (),
        }
    }
}


const DUMMY_CANVA_INFO:CanvaData=CanvaData{
    size: (0.,0.),
    position: (0.,0.),

    window_resolution:(0,0),
};