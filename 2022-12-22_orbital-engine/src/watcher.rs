use std::path::Path;

use notify::{RecursiveMode, Watcher as _};
use winit::event_loop::EventLoopProxy;

pub struct Watcher {
    _watcher: notify::RecommendedWatcher,
}

impl Watcher {
    pub fn new(proxy: EventLoopProxy<()>) -> anyhow::Result<Self> {
        let mut watcher = notify::recommended_watcher(move |_event| {
            proxy.send_event(()).unwrap();
        })?;
        watcher
            .watch(Path::new("../orbital-game"), RecursiveMode::Recursive)?;
        Ok(Self { _watcher: watcher })
    }
}
