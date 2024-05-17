use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug, Default, Serialize, PartialEq)]
pub struct GameStats {
    pub total_kills: u32,
    pub players: Vec<String>,
    pub kills: HashMap<String, i32>,
    pub kills_by_means: HashMap<String, u32>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Report {
    pub games: Vec<(String, GameStats)>,
    pub player_rankings: Vec<(String, i32)>,
    pub total_deaths_by_means: HashMap<String, u32>,
}

pub struct LogParser;

impl LogParser {
    pub fn parse_log(file_path: &str) -> Result<Report> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut games: Vec<(String, GameStats)> = Vec::new();
        let mut current_game = GameStats::default();
        let mut game_counter = 1;
        let mut current_players = HashMap::new();
        let mut in_game = false;
        let mut total_deaths_by_means = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            if line.contains("InitGame") {
                if in_game {
                    // Save the current game
                    games.push((format!("game_{}", game_counter), current_game));
                    game_counter += 1;
                    current_game = GameStats::default();
                    current_players.clear();
                }
                in_game = true;
            } else if line.contains("ShutdownGame") {
                if in_game {
                    // Save the current game
                    games.push((format!("game_{}", game_counter), current_game));
                    game_counter += 1;
                    current_game = GameStats::default();
                    current_players.clear();
                    in_game = false;
                }
            } else if in_game {
                if line.contains("Kill:") {
                    current_game.total_kills += 1;
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() < 6 {
                        continue; // Skip malformed lines
                    }
                    let killer_info = parts[5..].join(" ");
                    let killer_info_parts: Vec<&str> = killer_info.split(" killed ").collect();
                    if killer_info_parts.len() < 2 {
                        continue; // Skip malformed lines
                    }
                    let killer_name = clean_player_name(killer_info_parts[0]);
                    let killed_info_parts: Vec<&str> = killer_info_parts[1].split(" by ").collect();
                    if killed_info_parts.len() < 2 {
                        continue; // Skip malformed lines
                    }
                    let killed_name = clean_player_name(killed_info_parts[0]);
                    let means_of_death = killed_info_parts[1].to_string();

                    if killer_name != "<world>" {
                        *current_game.kills.entry(killer_name.clone()).or_insert(0) += 1;
                    } else {
                        *current_game.kills.entry(killed_name.clone()).or_insert(0) -= 1;
                    }

                    *current_game
                        .kills_by_means
                        .entry(means_of_death.clone())
                        .or_insert(0) += 1;
                    *total_deaths_by_means.entry(means_of_death).or_insert(0) += 1;

                    if killer_name != "<world>" && !current_players.contains_key(&killer_name) {
                        current_game.players.push(killer_name.clone());
                        current_players.insert(killer_name, true);
                    }
                    if !current_players.contains_key(&killed_name) {
                        current_game.players.push(killed_name.clone());
                        current_players.insert(killed_name, true);
                    }
                } else if line.contains("ClientUserinfoChanged:") {
                    let parts: Vec<&str> = line.split(' ').collect();
                    if parts.len() < 6 {
                        continue; // Skip malformed lines
                    }
                    let player_info = parts[5..].join(" ");
                    let player_info_parts: Vec<&str> = player_info.split('\\').collect();
                    if player_info_parts.len() < 2 {
                        continue; // Skip malformed lines
                    }
                    let player_name = clean_player_name(player_info_parts[1]);
                    if !current_players.contains_key(&player_name) {
                        current_game.players.push(player_name.clone());
                        current_players.insert(player_name, true);
                    }
                }
            }
        }

        if in_game {
            // Save the last game if it was not closed by "ShutdownGame"
            games.push((format!("game_{}", game_counter), current_game));
        }

        // Remove invalid player names and ensure consistency
        for (_, game) in games.iter_mut() {
            game.players
                .retain(|player| player != "t" && player != "<world>");
            game.kills
                .retain(|player, _| player != "t" && player != "<world>");
        }

        let player_rankings = Self::generate_rankings(&games);

        Ok(Report {
            games,
            player_rankings,
            total_deaths_by_means,
        })
    }

    fn generate_rankings(games: &[(String, GameStats)]) -> Vec<(String, i32)> {
        let mut player_rankings: HashMap<String, i32> = HashMap::new();
        for (_, stats) in games {
            for (player, kills) in &stats.kills {
                *player_rankings.entry(player.clone()).or_insert(0) += kills;
            }
        }

        let mut rankings: Vec<(String, i32)> = player_rankings.into_iter().collect();
        rankings.sort_by(|a, b| b.1.cmp(&a.1));

        rankings
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log_single_game() {
        let log_data = "\
        0:00 ------------------------------------------------------------\n\
        0:00 InitGame: \n\
        20:34 ClientConnect: 2\n\
        20:34 ClientUserinfoChanged: 2 n\\Isgalamido\\t\\0\n\
        20:37 ClientBegin: 2\n\
        21:42 Kill: 1022 2 22: <world> killed Isgalamido by MOD_TRIGGER_HURT\n\
        20:37 ShutdownGame:\n\
        0:00 ------------------------------------------------------------";

        let file_path = "test_logs/single_game.log";
        std::fs::create_dir_all("test_logs").unwrap();
        std::fs::write(file_path, log_data).unwrap();

        let report = LogParser::parse_log(file_path).unwrap();

        let expected_games = vec![(
            "game_1".to_string(),
            GameStats {
                total_kills: 1,
                players: vec!["Isgalamido".to_string()],
                kills: vec![("Isgalamido".to_string(), -1)].into_iter().collect(),
                kills_by_means: vec![("MOD_TRIGGER_HURT".to_string(), 1)]
                    .into_iter()
                    .collect(),
            },
        )];

        let expected_report = Report {
            games: expected_games,
            player_rankings: vec![("Isgalamido".to_string(), -1)],
            total_deaths_by_means: vec![("MOD_TRIGGER_HURT".to_string(), 1)]
                .into_iter()
                .collect(),
        };

        assert_eq!(report, expected_report);
    }

    #[test]
    fn test_parse_log_multiple_games() {
        let log_data = "\
        0:00 ------------------------------------------------------------\n\
        0:00 InitGame: \n\
        20:34 ClientConnect: 2\n\
        20:34 ClientUserinfoChanged: 2 n\\Isgalamido\\t\\0\n\
        20:37 ClientBegin: 2\n\
        21:42 Kill: 1022 2 22: <world> killed Isgalamido by MOD_TRIGGER_HURT\n\
        20:37 ShutdownGame:\n\
        0:00 ------------------------------------------------------------\n\
        0:00 InitGame: \n\
        21:07 Kill: 1022 2 22: <world> killed Isgalamido by MOD_FALLING\n\
        20:37 ShutdownGame:\n\
        0:00 ------------------------------------------------------------";

        let file_path = "test_logs/multiple_games.log";
        std::fs::write(file_path, log_data).unwrap();

        let report = LogParser::parse_log(file_path).unwrap();

        let expected_games = vec![
            (
                "game_1".to_string(),
                GameStats {
                    total_kills: 1,
                    players: vec!["Isgalamido".to_string()],
                    kills: vec![("Isgalamido".to_string(), -1)].into_iter().collect(),
                    kills_by_means: vec![("MOD_TRIGGER_HURT".to_string(), 1)]
                        .into_iter()
                        .collect(),
                },
            ),
            (
                "game_2".to_string(),
                GameStats {
                    total_kills: 1,
                    players: vec!["Isgalamido".to_string()],
                    kills: vec![("Isgalamido".to_string(), -1)].into_iter().collect(),
                    kills_by_means: vec![("MOD_FALLING".to_string(), 1)].into_iter().collect(),
                },
            ),
        ];

        let expected_report = Report {
            games: expected_games,
            player_rankings: vec![("Isgalamido".to_string(), -2)],
            total_deaths_by_means: vec![
                ("MOD_TRIGGER_HURT".to_string(), 1),
                ("MOD_FALLING".to_string(), 1),
            ]
            .into_iter()
            .collect(),
        };

        assert_eq!(report, expected_report);
    }
}
