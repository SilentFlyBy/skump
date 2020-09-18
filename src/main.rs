use clap::{App, Arg, ArgMatches};

fn main() {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
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
        .subcommand(App::new("ls").about("lists chats").alias("list"))
        .get_matches();

    match matches.subcommand() {
        ("ls", Some(_sub_m)) => list(_sub_m),
        _ => {}
    }
}

fn list(args: &ArgMatches) {
    println!("test")
}
