use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{error::AppError, player::Player};

#[derive(Serialize, Deserialize, Clone)]
pub struct Session {
    pub session_id: String,
    #[serde(skip)]
    pub players: HashMap<String, Player>
}

impl Session {
    pub fn new(session_id: String) -> Session {
        Session {
            session_id,
            players: HashMap::new()
        }
    }

    pub fn add_player(&mut self, player: Player) -> Result<&Player, AppError> {
        let guild_id = player.clone().guild_id;
        self.players.insert(guild_id.clone(), player);

        if let Some(p) = self.players.get(&guild_id) {
            return Ok(p);
        }
        Err(AppError::NotFound(String::from("Player not found.")))
    }

    pub async fn delete_player(&mut self, guild_id: String) -> Result<(), AppError> {
        if let Some(mut p) = self.players.remove(&guild_id) {
            p.disconnect().await;
        }
        
        Ok(())
    }
}
