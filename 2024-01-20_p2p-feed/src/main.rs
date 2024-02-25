use std::{
    fs::{self, File},
    io::Write,
    ops::Deref,
    path::Path,
    time::SystemTime,
};

use anyhow::Context;
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
        .map(switch(Item::store))
        .try_for_each(track(Item::print))?;

    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Item {
    pub _timestamp: u64,
    pub id: String,
    pub title: Option<String>,
    pub links: Vec<String>,
}

impl Item {
    pub fn from_entry(entry: Entry) -> anyhow::Result<Self> {
        let timestamp = SystemTime::UNIX_EPOCH
            .elapsed()?
            .as_nanos()
            .try_into()
            .context("Expected system time to be within ~500 years of epoch")?;
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

    pub fn store(self) -> anyhow::Result<Self> {
        let hash = {
            let mut hasher = blake3::Hasher::new();

            hasher.update(&self._timestamp.to_be_bytes());
            hasher.update(self.id.as_bytes());
            if let Some(title) = &self.title {
                hasher.update(title.as_bytes());
            }
            for link in &self.links {
                hasher.update(link.as_bytes());
            }

            hasher.finalize()
        };
        let toml = toml::ser::to_string_pretty(&self)
            .context("Serializing to TOML")?;

        let store = Path::new("store");
        fs::create_dir_all(store)?;

        let path = store.join(format!("{hash}.toml"));
        File::create(path)?.write_all(toml.as_bytes())?;

        Ok(self)
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
