use anyhow::{anyhow, Result};
use clap::ArgMatches;
use rusqlite::Connection;
use std::{fs::canonicalize, fs::metadata, path::PathBuf, process};

mod app;
mod db;
mod exit;
mod export;
mod templates;

fn main() {
    let matches = app::build_app().get_matches();

    let result = match matches.subcommand() {
        Some(("ls", _sub_m)) => list_command(_sub_m),
        Some(("export", _sub_m)) => export_command(_sub_m),
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
    let id: u32 = matches.value_of_t("id").unwrap();
    let db = Connection::open(full_path)?;
    let messages = db::get_messages(&db, id)?;
    let user = db::get_self_user(&db)?;
    let output = matches.value_of("output format").unwrap();

    match output {
        "console" => export::cli::to_cli(messages),
        "html" => export::html::to_html(messages, user),
        "pdf" => export::pdf::to_pdf(messages, user),
        _ => Ok(()),
    }
}

fn list_command(matches: &ArgMatches) -> Result<()> {
    let full_path = get_input_path(matches)?;
    let db = Connection::open(full_path)?;
    let conversations = db::get_conversations(&db)?;

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
