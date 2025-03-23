use core::f32;

use ball::Ball;
use canva::{Canva, CanvaDrawable};
use draw::Drawable;
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
mod draw;
mod canva;
mod constants;
mod vertex;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let (window, display) = backend::glutin::SimpleWindowBuilder::new()
        .with_title("Bouncing ball !")
        .build(&event_loop);

    let frag_shad = std::fs::read_to_string("./shaders/ball.frag").unwrap();
    let vert_shad = std::fs::read_to_string("./shaders/ball.vert").unwrap();
    let program = Program::from_source(&display, &vert_shad, &frag_shad, None).unwrap();
    let balls = vec![
        Ball::new(100., [200.; 2]),
        // Ball::new(50., [100.;2]),
    ];

    let canva = Canva::new(balls,(0.,0.), program);

    let mut app = App {
        canva,

        dt: 0.,
        time: std::time::Instant::now(),

        display,
        window,

        mouse_position: (0., 0.),
        mouse_cliking: false,
    };

    event_loop.set_control_flow(event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut app).unwrap();
}

struct App {
    canva: Canva<Ball>,

    dt: f32,
    time: std::time::Instant,

    display: Display<WindowSurface>,
    window: Window,

    mouse_position: (f32, f32),
    mouse_cliking: bool,
}

impl App {
    fn draw(&mut self) {
        let mut target = self.display.draw();

        target.clear_color(0., 0., 0., 1.);
        self.canva.draw(&self.display, &mut target);

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
            }
            WindowEvent::Moved(pos) => {
                println!("moved : {pos:?}");
                let [x, y]: [f32; 2] = pos.into();
                for balls in &mut self.canva.elements {
                    balls.position[0] -= x;
                    balls.position[1] -= y;
                }
            }

            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {


                let rel_position = (position.x as f32 / self.window.inner_size().width as f32, position.y as f32 / self.window.inner_size().height as f32);

                //dragging
                if self.mouse_cliking {
                    for ball in &mut self.canva.elements{
                        if ball.is_coord_in_relative(rel_position){
                            ball.on_drag(self.mouse_position.into(), rel_position.into());
                        }
                    }
                }

                self.mouse_position = rel_position;
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => match (button, state) {
                (MouseButton::Left, ElementState::Pressed) => { 
                    self.mouse_cliking = true;
                    if self.canva.is_coord_in_relative(self.mouse_position){
                        self.canva.on_click(self.mouse_position);
                    }
                },
                (MouseButton::Left, ElementState::Released) => {
                    self.mouse_cliking = false;
                    for ball in &mut self.canva.elements {
                        if ball.is_coord_in_relative(self.mouse_position) {
                            ball.on_click_release();
                        }
                    }
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
                self.canva.update(self.dt, &self.window);

                //draw
                self.draw();
            }
            _ => (),
        }
    }
}
