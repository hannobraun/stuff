mod item;
mod railway;

use std::ops::Deref;

use item::Item;
use railway::{switch, track};

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
        .map(switch(Item::store))
        .try_for_each(track(Item::print))?;

    Ok(())
}
