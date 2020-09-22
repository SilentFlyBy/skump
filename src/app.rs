use clap::{App, Arg};

pub fn build_app() -> App<'static> {
    let app = App::new("Skump - Skype dumper")
        .version("0.1")
        .author("Erik M. <erik@moldtmann.de>")
        .about("Tool for exporting skype chats to various sources")
        .subcommand(
            App::new("export")
                .about("Exports a chat")
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
                .arg(
                    Arg::with_name("id")
                        .about("ID of the conversation")
                        .long("id")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("ls")
                .about("lists chats")
                .alias("list")
                .arg(Arg::with_name("INPUT").index(1)),
        );

    app
}
