extern crate mpris;

use mpris::{Metadata, PlayerFinder};

fn main() {
    match retrieve_metadata() {
        Ok(metadata) => {
            println!("Metadata:\n{:#?}", metadata);
            println!("----\nRest of the metadata:\n{:#?}", metadata.rest_hash());
        }
        Err(error) => {
            println!("ERROR: {}", error);
            std::process::exit(1);
        }
    }
}

fn retrieve_metadata() -> Result<Metadata, String> {
    let player_finder =
        PlayerFinder::new().map_err(|e| format!("Could not connect to D-Bus: {}", e))?;

    let player = player_finder
        .find_active()
        .map_err(|e| format!("Could not find any player: {}", e))?;

    println!(
        "Found {identity} (on bus {bus_name})",
        bus_name = player.bus_name(),
        identity = player.identity(),
    );

    player
        .get_metadata()
        .map_err(|e| format!("Could not get metadata for player: {}", e))
}
