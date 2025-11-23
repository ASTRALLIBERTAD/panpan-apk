// build this with `--features desktop`
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;


fn main() {
let event_loop = EventLoop::new();
let window = WindowBuilder::new().with_title("PanPan (desktop)").build(&event_loop).unwrap();


// create GL context using glutin or winit+raw window handle + glow/gl
// For brevity this file shows high level flow only


// initialize engine after GL context is created
panpan::init();


event_loop.run(move |event, _, control_flow| {
*control_flow = ControlFlow::Poll;
match event {
Event::MainEventsCleared => {
// update
panpan::render();
// swap buffers
}
Event::WindowEvent { event, .. } => match event {
WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
_ => {}
},
_ => {}
}
});
}