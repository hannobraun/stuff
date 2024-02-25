use std::{ops::Deref, time::Instant};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let feed = reqwest::get("https://www.hannobraun.com/rss/")
        .await?
        .bytes()
        .await?;
    let feed = feed_rs::parser::parse(feed.deref())?;

    let items = feed.entries.into_iter().map(|entry| {
        let timestamp = Instant::now();
        let id = entry.id;
        let title = entry.title.map(|title| title.content);
        let links = entry.links.into_iter().map(|link| link.href).collect();

        Item {
            _timestamp: timestamp,
            id,
            title,
            links,
        }
    });

    for entry in items {
        println!("{}", entry.id);

        let title = entry
            .title
            .map(|title| format!("Title: {}", title))
            .unwrap_or_else(|| "no title".to_string());

        println!("- {title}");

        if entry.links.is_empty() {
            println!("- no links")
        } else {
            println!("- Links:");

            for link in entry.links {
                println!("  - {}", link);
            }
        }

        println!();
        println!();
    }

    Ok(())
}

struct Item {
    pub _timestamp: Instant,
    pub id: String,
    pub title: Option<String>,
    pub links: Vec<String>,
}
