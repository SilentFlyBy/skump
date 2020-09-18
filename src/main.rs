use clap::{App, Arg, ArgMatches};
mod list;

fn main() {
    let mut app = App::new("Skump - Skype dumper")
        .version("0.1")
        .author("Erik M. <erik@moldtmann.de>")
        .about("Tool for exporting skype chats to various sources")
        .arg(
            Arg::with_name("output format")
                .short('o')
                .long("output-format")
                .about("Sets the output format for a chat")
                .takes_value(true)
                .possible_values(&["console", "html", "pdf"])
                .default_value("console"),
        )
        .arg(
            Arg::with_name("INPUT")
                .about("Sets the input file to use")
                .index(1),
        )
        .subcommand(App::new("ls").about("lists chats").alias("list"));

    let matches = app.get_matches_mut();

    match matches.subcommand() {
        ("ls", Some(_sub_m)) => list_command(_sub_m).unwrap(),
        _ => main_command(&matches).unwrap(),
    };
}

fn list_command(_args: &ArgMatches) -> Result<(), ()> {
    println!("list");
    Ok(())
}

fn main_command(_args: &ArgMatches) -> Result<(), ()> {
    println!("main");
    Ok(())
}
