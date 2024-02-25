use std::ops::Deref;

use crate::{
    item::Item,
    railway::{switch, track},
};

pub async fn main() -> anyhow::Result<()> {
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
