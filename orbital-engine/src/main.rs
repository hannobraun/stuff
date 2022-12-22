mod event_loop;
mod renderer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    event_loop::run().await?;
    Ok(())
}