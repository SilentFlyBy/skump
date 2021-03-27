use anyhow::{Result};
use crate::db;

pub fn to_cli(messages: Vec<db::Message>) -> Result<()> {
    for m in messages {
        println!("{}: {}", m.author, m.body);
    }

    Ok(())
}