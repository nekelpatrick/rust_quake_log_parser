use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug, Default, Serialize)]
pub struct GameStats {
    pub total_kills: u32,
    pub players: Vec<String>,
    pub kills: HashMap<String, i32>,
    pub kills_by_means: HashMap<String, u32>,
}

pub struct LogParser;

impl LogParser {
    pub fn parse_log(file_path: &str) -> Result<GameStats> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut game_stats = GameStats::default();
        let mut players_set = std::collections::HashSet::new();

        for line in reader.lines() {
            let line = line?;
            if line.contains("Kill:") {
                game_stats.total_kills += 1;
                let parts: Vec<&str> = line.split_whitespace().collect();
                let killer_id = parts[1];
                let killed_id = parts[2];
                let means_of_death = parts[4];
                let killer_name = parts[5..]
                    .join(" ")
                    .split(" killed ")
                    .next()
                    .unwrap()
                    .trim()
                    .to_string();
                let killed_name = parts[5..]
                    .join(" ")
                    .split(" killed ")
                    .nth(1)
                    .unwrap()
                    .split(" by ")
                    .next()
                    .unwrap()
                    .trim()
                    .to_string();

                *game_stats
                    .kills_by_means
                    .entry(means_of_death.to_string())
                    .or_insert(0) += 1;

                if killer_name != "<world>" {
                    *game_stats.kills.entry(killer_name.clone()).or_insert(0) += 1;
                }
                *game_stats.kills.entry(killed_name.clone()).or_insert(0) -= 1;

                if killer_name != "<world>" {
                    players_set.insert(killer_name);
                }
                players_set.insert(killed_name);
            }
        }

        game_stats.players = players_set.into_iter().collect();
        Ok(game_stats)
    }
}
