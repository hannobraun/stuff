#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let response = reqwest::get("https://hanno.braun-odw.eu/atom.xml").await?;
    dbg!(response);
    Ok(())
}
