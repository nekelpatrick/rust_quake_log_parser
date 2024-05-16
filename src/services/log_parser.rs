use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug, Default, Serialize)]
pub struct GameStats {
    pub total_kills: u32,
    pub players: Vec<String>,
    pub kills: HashMap<String, i32>,
}

pub struct LogParser;

impl LogParser {
    pub fn parse_log(file_path: &str) -> Result<GameStats> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut game_stats = GameStats::default();
        for line in reader.lines() {
            let line = line?;
            if line.contains("Kill:") {
                game_stats.total_kills += 1;
                let parts: Vec<&str> = line.split(' ').collect();
                let killer_info_str = parts[5..].join(" ");
                let killer_info: Vec<&str> = killer_info_str.split(" by ").collect();
                let killer_name = killer_info[0].to_string();
                let killed_name = killer_info[1].to_string();

                if killer_name != "<world>" {
                    *game_stats.kills.entry(killer_name.clone()).or_insert(0) += 1;
                }
                *game_stats.kills.entry(killed_name.clone()).or_insert(0) -= 1;

                if !game_stats.players.contains(&killer_name) && killer_name != "<world>" {
                    game_stats.players.push(killer_name);
                }
                if !game_stats.players.contains(&killed_name) {
                    game_stats.players.push(killed_name);
                }
            }
        }

        Ok(game_stats)
    }
}
