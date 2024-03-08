use cali::parser::Parser;
use log::error;

mod backend;

fn main() {
    pretty_env_logger::formatted_timed_builder().filter_level(log::LevelFilter::Info).parse_env("LOG").init();
    #[rustfmt::skip]
    let mut parser = Parser::new()
        .add_arg("a", "add", "Adds / replaces binding with optional index", true, true)
        .add_arg("r", "remove", "Removes binding with index", true, false)
        .add_arg("e", "enable", "Enables binding with index", true, false)
        .add_arg("d", "disable", "Disables binding with index", true, false)
        .add_arg("l", "list", "Lists all bindings", false, false)
        .add_arg("h", "help", "Prints this help dialogue", false, false);

    parser.parse().err().map(|err| {
        error!("{}", err);
        std::process::exit(-1)
    });

    if let Some(_) = parser.get_parsed_argument_long("help") {
        parser
            .get_arguments()
            .iter()
            .for_each(|arg| println!("\t{}", arg.to_string()));
        return;
    }
}
