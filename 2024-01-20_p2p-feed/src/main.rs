use std::{ops::Deref, time::SystemTime};

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
        .try_for_each(track(Item::print))?;

    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Item {
    pub _timestamp: u128,
    pub id: String,
    pub title: Option<String>,
    pub links: Vec<String>,
}

impl Item {
    pub fn from_entry(entry: Entry) -> anyhow::Result<Self> {
        let timestamp = SystemTime::UNIX_EPOCH.elapsed()?.as_nanos();
        let id = entry.id;
        let title = entry.title.map(|title| title.content);
        let links = entry.links.into_iter().map(|link| link.href).collect();

        Ok(Item {
            _timestamp: timestamp,
            id,
            title,
            links,
        })
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

pub fn switch<A, B, E>(
    mut f: impl FnMut(A) -> Result<B, E>,
) -> impl FnMut(Result<A, E>) -> Result<B, E> {
    move |res| match res {
        Ok(a) => f(a),
        Err(err) => Err(err),
    }
}

pub fn track<A, B, E>(
    mut f: impl FnMut(A) -> B,
) -> impl FnMut(Result<A, E>) -> Result<B, E> {
    move |res| match res {
        Ok(a) => Ok(f(a)),
        Err(err) => Err(err),
    }
}
