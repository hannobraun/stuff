use std::path::Path;

use notify::{RecursiveMode, Watcher as _};

pub struct Watcher {
    _watcher: notify::RecommendedWatcher,
}

impl Watcher {
    pub fn new() -> anyhow::Result<Self> {
        let mut watcher = notify::recommended_watcher(|event| {
            println!("Game code changed: {event:?}");
        })?;
        watcher
            .watch(Path::new("../orbital-game"), RecursiveMode::Recursive)?;
        Ok(Self { _watcher: watcher })
    }
}
