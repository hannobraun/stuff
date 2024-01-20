#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let feed = reqwest::get("https://hanno.braun-odw.eu/atom.xml")
        .await?
        .text()
        .await?;
    dbg!(feed);
    Ok(())
}
