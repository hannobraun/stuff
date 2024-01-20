#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let feed = reqwest::get("https://hanno.braun-odw.eu/atom.xml")
        .await?
        .text()
        .await?;
    let feed = feed_rs::parser::parse(feed.as_bytes())?;
    dbg!(feed);
    Ok(())
}
