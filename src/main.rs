use anyhow::{anyhow, Result};
use clap::ArgMatches;
use rusqlite::{Connection, NO_PARAMS};
use std::{fs::canonicalize, fs::metadata, path::PathBuf, process};

mod app;
mod db;
mod exit;
mod list;

struct Chat {
    id: u32,
    name: String,
}

fn main() {
    let matches = app::build_app().get_matches();

    let result = match matches.subcommand() {
        ("ls", Some(_sub_m)) => list_command(_sub_m),
        ("export", Some(_sub_m)) => export_command(_sub_m),
        _ => Ok(()),
    };

    match result {
        Ok(_) => process::exit(exit::ExitCode::Ok.into()),
        Err(e) => {
            println!("{}", e.to_string());
            process::exit(exit::ExitCode::Error.into());
        }
    }
}

fn export_command(matches: &ArgMatches) -> Result<()> {
    let full_path = get_input_path(matches)?;
    Ok(())
}

fn list_command(matches: &ArgMatches) -> Result<()> {
    let full_path = get_input_path(matches)?;
    let db = Connection::open(full_path)?;
    let mut select = db.prepare(db::GET_CONVERSATIONS)?;

    let conversations: Vec<Chat> = select
        .query_map(NO_PARAMS, |row| {
            Ok(Chat {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?
        .map(|c| c.unwrap())
        .collect();

    for con in conversations {
        println!("{} {}", con.id, con.name);
    }

    Ok(())
}

fn get_input_path(matches: &ArgMatches) -> Result<PathBuf> {
    let input = matches.value_of("INPUT");
    if input.is_none() {
        return Err(anyhow!("No value given for INPUT"));
    }

    let path = PathBuf::from(input.unwrap());
    let full_path = canonicalize(&path)?;
    let md = metadata(&full_path)?;
    if !md.is_file() {
        return Err(anyhow!(
            "The input path '{}' is not a file.",
            full_path.to_string_lossy()
        ));
    }

    Ok(full_path)
}
