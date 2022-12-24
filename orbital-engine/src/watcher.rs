use std::path::Path;

use notify::{RecursiveMode, Watcher as _};

pub struct Watcher {
    _watcher: notify::RecommendedWatcher,
}

pub fn start() -> anyhow::Result<Watcher> {
    let mut watcher = notify::recommended_watcher(|event| {
        println!("Game code changed: {event:?}");
    })?;
    watcher.watch(Path::new("../orbital-game"), RecursiveMode::Recursive)?;
    Ok(Watcher { _watcher: watcher })
}
