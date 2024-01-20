use std::ops::Deref;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let feed = reqwest::get("https://hanno.braun-odw.eu/atom.xml")
        .await?
        .bytes()
        .await?;
    let feed = feed_rs::parser::parse(feed.deref())?;

    for entry in feed.entries {
        println!("{}", entry.id);

        let title = entry
            .title
            .map(|title| format!("Title: {}", title.content))
            .unwrap_or_else(|| "no title".to_string());

        println!("- {title}");

        if entry.links.is_empty() {
            println!("- no links")
        } else {
            println!("- Links:");

            for link in entry.links {
                println!("  - {}", link.href);
            }
        }

        println!();
        println!();
    }

    Ok(())
}
