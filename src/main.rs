use backend::upnp_funcs::get_gateways;
use cali::parser::Parser;
use igd_next::SearchOptions;
use log::error;

mod backend;

fn main() -> Result<(), String> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .parse_env("LOG")
        .init();
    #[rustfmt::skip]
    let mut parser = Parser::new()
        .add_arg("a", "add", "Adds / replaces binding with optional index", true, true)
        .add_arg("r", "remove", "Removes binding with index", true, false)
        .add_arg("e", "enable", "Enables binding with index", true, false)
        .add_arg("d", "disable", "Disables binding with index", true, false)
        .add_arg("l", "list", "Lists all bindings", false, false)
        .add_arg("h", "help", "Prints this help dialogue", false, false);

    parser.parse().map_err(|err| err.to_string())?;

    if let Some(_) = parser.get_parsed_argument_long("help") {
        parser
            .get_arguments()
            .iter()
            .for_each(|arg| println!("\t{}", arg.to_string()));
        return Ok(());
    }

    let gateway =
        igd_next::search_gateway(SearchOptions::default()).map_err(|err| err.to_string())?;

    
}
