mod event_loop;
mod host;
mod renderer;
mod watcher;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    event_loop::run().await?;
    Ok(())
}
