use std::{
    fs::OpenOptions,
    io::{Read, Write},
    net::{IpAddr, SocketAddr},
};

use igd_next::{Gateway, PortMappingProtocol};
use log::{debug, error, info};

use super::upnp_structs::Mapping;

/// adds a mapping to the config
pub fn add_mapping() -> Result<(), String> {
    let mut mappings = read_mappings()?;
    // create new mapping form user input
    mappings.push(Mapping::new(
        // get internal ip from user
        get_input("Internal IP: ")
            .map_err(|err| format!("Could not read input: {err}"))?
            .parse::<IpAddr>()
            .map_err(|err| format!("Could not parse input to IpAddr: {err}"))?,
        // get internal port from user
        get_input("Internal Port: ")
            .map_err(|err| format!("Could not read input: {err}"))?
            .parse::<u16>()
            .map_err(|err| format!("Could not parse input to u16: {err}"))?,
        // get external port from user
        get_input("External Port: ")
            .map_err(|err| format!("Could not read input: {err}"))?
            .parse::<u16>()
            .map_err(|err| format!("Could not parse input to u16: {err}"))?,
        // get description from user
        get_input("Description: ").map_err(|err| format!("Could not read input: {err}"))?,
    ));

    // write updated config to file
    write_mappings(mappings)?;
    Ok(())
}

/// removes a mapping from the config
pub fn remove_mapping(gateway: &Gateway, index: Option<usize>) -> Result<(), String> {
    let mut mappings = read_mappings()?;
    if mappings.is_empty() {
        info!("No mappings to show");
        return Ok(());
    }

    // decide wether to use supplied index or prompt user
    let remove_idx = if index.is_none() {
        debug!("No index supplied, prompting user...");
        mappings.iter().enumerate().for_each(|(idx, mapping)| {
            println!(
                "{}: {}:{: <5}\t->\t{}:{: <5}\t{}",
                idx + 1,
                mapping.dest_ip,
                mapping.dest_port,
                gateway.addr.ip(),
                mapping.ext_port,
                mapping.description
            )
        });
        get_input(format!("Mapping to delete (1 - {}): ", mappings.len()).as_str())
            .map_err(|err| format!("Could not read input: {err}"))?
            .parse::<usize>()
            .map_err(|err| format!("Failed to parse to usize: {err}"))?
    } else {
        debug!("Using supplied index...");
        index.unwrap_or_default()
    };

    if remove_idx <= mappings.len() && remove_idx > 0 {
        mappings.remove(remove_idx - 1);
    } else {
        return Err("Index out of bounds".to_string());
    }

    // write updated config to file
    write_mappings(mappings)?;
    Ok(())
}

/// lists all mappings in the config
pub fn list_mappings(gateway: &Gateway) -> Result<(), String> {
    let mappings = read_mappings()?;
    mappings.iter().enumerate().for_each(|(idx, mapping)| {
        println!(
            "{}: {}:{: <5}\t->\t{}:{: <5}\t{}",
            idx + 1,
            mapping.dest_ip,
            mapping.dest_port,
            gateway.addr.ip(),
            mapping.ext_port,
            mapping.description
        )
    });
    Ok(())
}

/// enables a mapping from the config
pub fn enable_mapping(gateway: &Gateway, index: Option<usize>) -> Result<(), String> {
    let mappings = read_mappings()?;

    // decide wether to use supplied index or prompt user
    let activation_idx = if index.is_none() {
        debug!("No index supplied, prompting user...");
        mappings.iter().enumerate().for_each(|(idx, mapping)| {
            println!(
                "{}: {}:{: <5}\t->\t{}:{: <5}\t{}",
                idx + 1,
                mapping.dest_ip,
                mapping.dest_port,
                gateway.addr.ip(),
                mapping.ext_port,
                mapping.description
            )
        });
        get_input(format!("Mapping to delete (1 - {}): ", mappings.len()).as_str())
            .map_err(|err| format!("Could not read input: {err}"))?
            .parse::<usize>()
            .map_err(|err| format!("Failed to parse to usize: {err}"))?
    } else {
        debug!("Using supplied index...");
        index.unwrap_or_default()
    };

    let mapping = match mappings.get(activation_idx - 1) {
        Some(mapping) => mapping,
        None => return Err("Index out of bounds".to_string()),
    };

    debug!("Sending request to gateway...");
    gateway
        .add_port(
            PortMappingProtocol::TCP,
            mapping.ext_port,
            SocketAddr::new(mapping.dest_ip, mapping.dest_port),
            0,
            &mapping.description,
        )
        .map_err(|err| format!("Failed to request mapping: {err}"))?;

    gateway
        .add_port(
            PortMappingProtocol::UDP,
            mapping.ext_port,
            SocketAddr::new(mapping.dest_ip, mapping.dest_port),
            0,
            &mapping.description,
        )
        .map_err(|err| format!("Failed to request mapping: {err}"))?;

    Ok(())
}

/// disables a mapping from the config
pub fn disable_mapping(gateway: &Gateway, index: Option<usize>) -> Result<(), String> {
    Ok(())
}

/// loads the config or creates config if not present
pub fn get_config(truncate: bool) -> Result<std::fs::File, std::io::Error> {
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(truncate)
        .open(crate::CONF_FILE)
}

pub fn read_mappings() -> Result<Vec<Mapping>, String> {
    debug!("Reading and parsing config from file...");
    let mut config = get_config(false).map_err(|err| err.to_string())?;
    let mut config_content = String::new();
    config
        .read_to_string(&mut config_content)
        .map_err(|err| err.to_string())?;
    Ok(serde_json::from_str::<Vec<Mapping>>(&config_content).unwrap_or_default())
}
pub fn write_mappings(mappings: Vec<Mapping>) -> Result<(), String> {
    let mut config = get_config(true).map_err(|err| err.to_string())?;
    config
        .write_all(
            serde_json::to_string_pretty(&mappings)
                .map_err(|err| err.to_string())?
                .as_bytes(),
        )
        .map_err(|err| err.to_string())?;
    Ok(())
}

/// reads input after printing out the prompt
pub fn get_input(prompt: &str) -> Result<String, std::io::Error> {
    let mut input = String::new();
    print!("{prompt}");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}
