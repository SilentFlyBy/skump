use anyhow::{anyhow, Result};
use clap::ArgMatches;
use rusqlite::{params, Connection, NO_PARAMS};
use std::{fs::canonicalize, fs::metadata, path::Path, path::PathBuf, process};

mod app;
mod db;
mod exit;
mod list;

struct Chat {
    Id: u32,
    Name: String,
}

fn main() {
    let matches = app::build_app().get_matches();

    let result = match matches.subcommand() {
        ("ls", Some(_sub_m)) => list_command(_sub_m),
        _ => main_command(&matches),
    };

    match result {
        Ok(_) => process::exit(exit::ExitCode::Ok.into()),
        Err(e) => {
            println!("{}", e.to_string());
            process::exit(exit::ExitCode::Error.into());
        }
    }
}

fn list_command(matches: &ArgMatches) -> Result<()> {
    if let Some(input) = matches.value_of("INPUT") {
        let path = PathBuf::from(input);
        let full_path = canonicalize(&path)?;
        let md = metadata(&full_path)?;
        if !md.is_file() {
            return Err(anyhow!(
                "The input path '{}' is not a file.",
                full_path.to_string_lossy()
            ));
        }

        let db = Connection::open(full_path)?;
        let mut select = db.prepare(db::GET_CONVERSATIONS)?;

        let conversations: Vec<Chat> = select
            .query_map(NO_PARAMS, |row| {
                Ok(Chat {
                    Id: row.get(0)?,
                    Name: row.get(1)?,
                })
            })?
            .map(|c| c.unwrap())
            .collect();

        for con in conversations {
            println!("{} {}", con.Id, con.Name);
        }
    }

    Ok(())
}

fn main_command(_args: &ArgMatches) -> Result<()> {
    println!("main");
    Ok(())
}
