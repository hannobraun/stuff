use std::ops::Deref;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let feed = reqwest::get("https://hanno.braun-odw.eu/atom.xml")
        .await?
        .bytes()
        .await?;
    let feed = feed_rs::parser::parse(feed.deref())?;

    for entry in feed.entries {
        dbg!(entry);
    }

    Ok(())
}
