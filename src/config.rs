use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub no_of_games: usize,
    pub initial_games: Vec<Vec<i64>>,
    pub seed: Option<u64>,       // Optional random seed
    pub max_number: i64,         // Maximum playable number
    pub min_desired_number: i64, // Minimum number desired in a valid game
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_can_parse_all_fields() {
        // Sample TOML string that includes all the fields used in Config
        let toml_str = r#"
            no_of_games = 3
            initial_games = [[1, 2, 3], [4, 5, 6]]
            seed = 12345
            max_number = 49
            min_desired_number = 10
        "#;

        // Try to parse the TOML string into our Config struct
        let parsed_config: Config =
            toml::from_str(toml_str).expect("Failed to parse TOML string into Config");

        // Verify that each field matches what we expect
        assert_eq!(parsed_config.no_of_games, 3);
        assert_eq!(
            parsed_config.initial_games,
            vec![vec![1, 2, 3], vec![4, 5, 6]]
        );
        assert_eq!(parsed_config.seed, Some(12345));
        assert_eq!(parsed_config.max_number, 49);
        assert_eq!(parsed_config.min_desired_number, 10);
    }
}
