use serde::{Deserialize, Serialize};

use super::{error::AppError, track::Track};

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub guild_id: String,
    pub track: Option<Track>,
    pub volume: usize,
    pub paused: bool,
    pub state: PlayerState,
    pub voice: Option<PlayerVoice>,
}

impl Player {
    pub fn new(
        guild_id: String,
        track: Option<Track>,
        volume: usize,
        paused: bool,
        state: PlayerState,
        voice: Option<PlayerVoice>,
    ) -> Player {
        Player {
            guild_id,
            track,
            volume,
            paused,
            state,
            voice,
        }
    }

    pub async fn edit_voice_state(
        &mut self,
        token: String,
        endpoint: String,
        session_id: String
    ) -> Result<(), AppError> {
        self.voice = Some(PlayerVoice::new(token, endpoint, session_id));

        Ok(())
    }

    pub async fn connect(
        &mut self
    ) -> Result<(), AppError> {
        todo!()
    }

    pub async fn disconnect(
        &mut self
    ) -> Result<(), AppError> {
        todo!()
    }

}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerState {
    time: usize,
    position: usize,
    connected: bool,
    ping: i64,
}

impl PlayerState {
    pub fn new(time: usize, position: usize, connected: bool, ping: i64) -> PlayerState {
        PlayerState {
            time,
            position,
            connected,
            ping,
        }
    }

    pub fn new_empty() -> PlayerState {
        Self::new(
            0,
            0,
            false,
            -1
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerVoice {
    token: String,
    endpoint: String,
    session_id: String,
}

impl PlayerVoice {
    pub fn new(token: String, endpoint: String, session_id: String) -> PlayerVoice {
        PlayerVoice {
            token,
            endpoint,
            session_id,
        }
    }

    pub fn new_empty() -> PlayerVoice {
        Self::new(
            String::new(),
            String::new(),
            String::new(),
        )
    }
}
