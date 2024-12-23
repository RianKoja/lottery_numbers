mod config;
mod custom_utils;
mod data_structures;

use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_file("config.toml")?;
    let mut wtr = csv::Writer::from_path("optimized_games.csv")?;
    let mut games = config.initial_games.clone();
    let mut game_set = data_structures::NumberSet::new();
    let mut triplet_set = data_structures::NumberSet::new();

    // Instantiate the invalidate_game closure
    let invalidate_game =
        custom_utils::create_invalidate_game(config.min_desired_number, config.max_number);

    // initialize the game_set and triplet_set with the initial games:
    for game in games.clone() {
        // check if game is valid:
        if invalidate_game(game.as_ref()) {
            // Game is not valid, inform the game:
            panic!("Invalid given at input found! Game: {:?}", game);
        }
        // Convert game to number:
        let game_no = custom_utils::game2enum(game.clone());
        // Convert to triplets:
        let triplets = custom_utils::game2triplets(game.clone());
        // Convert triplets to numbers:
        let triplet_nos = triplets
            .iter()
            .map(|triplet| custom_utils::triplet2enum(triplet.clone()))
            .collect::<Vec<_>>();
        // Try to insert triplets into triplet_set:
        if !triplet_set.check_and_insert_all(triplet_nos) {
            // If insertion fails, then a repeated triplet was found, should not happen here! Inform the triplets: and game:
            panic!(
                "Repeated triplet found! This should not happen! Game: {:?}, Triplets: {:?}",
                game, triplets
            );
        }
        // Add game to game_set:
        game_set.add_number(game_no);
    }

    // Create random number generator for combinadics
    let mut rng =
        custom_utils::create_combinadic_rng(config.seed.unwrap_or(12345), config.max_number, 6);

    while games.len() < config.no_of_games {
        let game_no = rng();
        let game = custom_utils::enum2game(game_no);

        if !game_set.add_number(game_no) || invalidate_game(&game) {
            continue;
        }

        let triplets = custom_utils::game2triplets(game.clone());
        let triplet_nos = triplets
            .iter()
            .map(|triplet| custom_utils::triplet2enum(triplet.clone()))
            .collect::<Vec<_>>();

        if !triplet_set.check_and_insert_all(triplet_nos) {
            continue;
        }

        games.push(game);
    }

    for row in games {
        let string_row: Vec<String> = row.iter().map(|item| item.to_string()).collect();
        wtr.write_record(&string_row)?;
    }

    game_set.save_to_file("games.csv")?;
    triplet_set.save_to_file("triplet_set.log")?;
    wtr.flush()?;

    Ok(())
}
