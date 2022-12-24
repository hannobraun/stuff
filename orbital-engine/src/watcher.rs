use std::path::Path;

use notify::{RecursiveMode, Watcher};

pub fn start() -> anyhow::Result<notify::RecommendedWatcher> {
    let mut watcher = notify::recommended_watcher(|event| {
        println!("Game code changed: {event:?}");
    })?;
    watcher.watch(Path::new("../orbital-game"), RecursiveMode::Recursive)?;
    Ok(watcher)
}
