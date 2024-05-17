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
    pub fn parse_log(file_path: &str) -> Result<HashMap<String, GameStats>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut games: HashMap<String, GameStats> = HashMap::new();
        let mut current_game = GameStats::default();
        let mut game_counter = 1;
        let mut current_players = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            if line.contains("InitGame") {
                if current_game.total_kills > 0 || !current_game.players.is_empty() {
                    games.insert(format!("game_{}", game_counter), current_game);
                    game_counter += 1;
                    current_game = GameStats::default();
                    current_players.clear();
                }
            } else if line.contains("Kill:") {
                current_game.total_kills += 1;
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 6 {
                    continue; // Skip malformed lines
                }
                let killer_id = parts[1];
                let killed_id = parts[2];
                let killer_name = if killer_id == "1022" {
                    "<world>".to_string()
                } else {
                    current_players.get(killer_id).cloned().unwrap_or_default()
                };
                let killed_name = current_players.get(killed_id).cloned().unwrap_or_default();

                if killer_name != "<world>" {
                    *current_game.kills.entry(killer_name.clone()).or_insert(0) += 1;
                } else {
                    *current_game.kills.entry("<world>".to_string()).or_insert(0) += 1;
                }
                *current_game.kills.entry(killed_name.clone()).or_insert(0) -= 1;

                if killer_name != "<world>" && !current_game.players.contains(&killer_name) {
                    current_game.players.push(killer_name.clone());
                }
                if !current_game.players.contains(&killed_name) {
                    current_game.players.push(killed_name.clone());
                }
            } else if line.contains("ClientUserinfoChanged:") {
                let parts: Vec<&str> = line.split(' ').collect();
                if parts.len() < 6 {
                    continue; // Skip malformed lines
                }
                let player_id = parts[1];
                let player_info = parts[5..].join(" ");
                let player_info_parts: Vec<&str> = player_info.split('\\').collect();
                if player_info_parts.len() < 2 {
                    continue; // Skip malformed lines
                }
                let player_name = clean_player_name(player_info_parts[1]);
                current_players.insert(player_id.to_string(), player_name.clone());
                if !current_game.players.contains(&player_name) {
                    current_game.players.push(player_name);
                }
            }
        }

        if current_game.total_kills > 0 || !current_game.players.is_empty() {
            games.insert(format!("game_{}", game_counter), current_game);
        }

        Ok(games)
    }
}

fn clean_player_name(name: &str) -> String {
    let name = name.trim();
    if name.contains(':') {
        name.split(':').nth(1).unwrap_or(name).trim().to_string()
    } else {
        name.to_string()
    }
}
