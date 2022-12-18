use winit::event_loop::EventLoop;

pub fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();

    event_loop.run(|event, _, _| match event {
        _ => {
            dbg!(event);
        }
    })
}
