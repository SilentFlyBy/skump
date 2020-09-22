use anyhow::{anyhow, Result};
use clap::ArgMatches;
use rusqlite::{params, Connection};
use std::{fs::metadata, path::Path};

mod app;
mod list;

struct Chat {
    Name: String,
}

fn main() {
    let matches = app::build_app().get_matches();

    match matches.subcommand() {
        ("ls", Some(_sub_m)) => list_command(_sub_m).unwrap(),
        _ => main_command(&matches).unwrap(),
    };
}

fn list_command(matches: &ArgMatches) -> Result<()> {
    if let Some(input) = matches.value_of("INPUT") {
        let input = Path::new(input);
        let md = metadata(input)?;
        if !md.is_file() {
            return Err(anyhow!(
                "The input path '{}' is not a file.",
                input.to_string_lossy()
            ));
        }

        let db = Connection::open(input)?;
        let mut select = db.prepare("")?;

        let r: Vec<Chat> = select
            .query_map(params![], |row| {
                Ok(Chat {
                    Name: String::from("Erik"),
                })
            })?
            .map(|c| c.unwrap())
            .collect();
    }

    Ok(())
}

fn main_command(_args: &ArgMatches) -> Result<()> {
    println!("main");
    Ok(())
}
