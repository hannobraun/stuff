use std::{ops::Deref, time::Instant};

use feed_rs::model::Entry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let feed = reqwest::get("https://www.hannobraun.com/rss/")
        .await?
        .bytes()
        .await?;
    let feed = feed_rs::parser::parse(feed.deref())?;

    feed.entries
        .into_iter()
        .map(Item::from_entry)
        .for_each(|item| item.print());

    Ok(())
}

struct Item {
    pub _timestamp: Instant,
    pub id: String,
    pub title: Option<String>,
    pub links: Vec<String>,
}

impl Item {
    pub fn from_entry(entry: Entry) -> Self {
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
    }

    pub fn print(self) {
        println!("{}", self.id);

        let title = self
            .title
            .map(|title| format!("Title: {}", title))
            .unwrap_or_else(|| "no title".to_string());

        println!("- {title}");

        if self.links.is_empty() {
            println!("- no links")
        } else {
            println!("- Links:");

            for link in self.links {
                println!("  - {}", link);
            }
        }

        println!();
        println!();
    }
}
