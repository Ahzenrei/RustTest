use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

fn main() {
    let window_builder = WindowBuilder::new(); 
    let mut event_loop = EventLoop::new();
    let _window = window_builder.build(&event_loop).expect("window");

    let mut running = true;

    while running {
        event_loop.run_return(|event, _, control_flow| {
            match &event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        running = false;
                    }
                    _ => {}
                },
                _ => (),
            }
        });
    }
}