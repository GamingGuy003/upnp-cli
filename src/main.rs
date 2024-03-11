use cali::parser::Parser;
use igd_next::SearchOptions;
use log::{info, warn};

use crate::backend::upnp_funcs::{
    add_mapping, disable_mapping, enable_mapping, list_mappings, remove_mapping,
};

mod backend;

static CONF_FILE: &str = "mappings.json";

fn main() -> Result<(), String> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .parse_env("LOG")
        .init();
    #[rustfmt::skip]
    let mut parser = Parser::new()
        .add_arg("a", "add", "Adds a binding", false, false)
        .add_arg("r", "remove", "Remove a binding", true, true)
        .add_arg("e", "enable", "Enables binding with index", true, true)
        .add_arg("d", "disable", "Disables binding with index", true, true)
        .add_arg("l", "list", "Lists all bindings", false, false)
        .add_arg("h", "help", "Prints this help dialogue", false, false);

    parser.parse().map_err(|err| err.to_string())?;

    if parser.get_parsed_argument_long("help").is_some() {
        parser
            .get_arguments()
            .iter()
            .for_each(|arg| println!("\t{arg}"));
        return Ok(());
    }

    info!("Trying to find gateway...");

    let gateway =
        igd_next::search_gateway(SearchOptions::default()).map_err(|err| err.to_string())?;

    info!("Found gateway {}", gateway.addr);

    for parsed_argument in parser.get_parsed_arguments() {
        match parsed_argument {
            _ if parsed_argument.long_matches("add") => add_mapping()?,
            _ if parsed_argument.long_matches("remove") => {
                let mut index = None;
                if let Some(value) = parsed_argument.value {
                    index = Some(
                        value
                            .parse::<usize>()
                            .map_err(|err| format!("User did not provide valid usize: {err}"))?,
                    );
                }
                remove_mapping(&gateway, index)?
            }
            _ if parsed_argument.long_matches("enable") => {
                let mut index = None;
                if let Some(value) = parsed_argument.value {
                    index = Some(
                        value
                            .parse::<usize>()
                            .map_err(|err| format!("User did not provide valid usize: {err}"))?,
                    );
                }
                enable_mapping(&gateway, index)?
            }
            _ if parsed_argument.long_matches("disable") => {
                let mut index = None;
                if let Some(value) = parsed_argument.value {
                    index = Some(
                        value
                            .parse::<usize>()
                            .map_err(|err| format!("User did not provide valid usize: {err}"))?,
                    );
                }
                disable_mapping(&gateway, index)?
            }
            _ if parsed_argument.long_matches("list") => list_mappings(&gateway)?,
            _ => warn!(
                "Argument {} not implemented",
                parsed_argument
                    .defined_argument
                    .unwrap_or_default()
                    .to_string()
            ),
        }
    }
    Ok(())
}
