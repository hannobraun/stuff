use winit::event_loop::EventLoop;

pub fn run() {
    let event_loop = EventLoop::new();

    event_loop.run(|event, _, _| {
        dbg!(event);
    })
}
