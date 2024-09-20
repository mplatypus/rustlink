use std::{collections::HashMap, sync::Arc};

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use secrecy::SecretString;

use dotenvy::from_filename;
use tokio::sync::Mutex;

use super::{error::AppError, player::{Player, PlayerState}, session::Session};

pub type App = Arc<Mutex<ApplicationState>>;

pub struct ApplicationState {
    pub sessions: HashMap<String, Session>,
    pub config: Config,
}

impl ApplicationState {
    pub fn new() -> Arc<Mutex<ApplicationState>> {
        let config = Config::from_env();

        Arc::new(Mutex::new(ApplicationState {
            sessions: HashMap::new(),
            config,
        }))
    }

    pub fn get_session(&mut self, session_id: String) -> Option<&Session> {
        self.sessions.get(&session_id)
    }

    pub fn new_session(&mut self) -> &Session {
        let random_bytes: Vec<u8> = (0..24).map(|_| rand::random::<u8>()).collect();
        let session_id = URL_SAFE.encode(&random_bytes);
        let session = Session::new(session_id.clone());
        self.sessions.insert(session_id.clone(), session);

        self.sessions.get(&session_id).expect("Session must exist")
    }

    pub fn get_players(&self, session_id: String) -> Vec<Player> {
        let session = self.sessions.get(&session_id);

        if let Some(s) = session {
            s.players.values().cloned().collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_player(&mut self, session_id: String, guild_id: String) -> Option<&Player> {
        let session = self.sessions.get(&session_id);

        if let Some(s) = session {
            s.players.get(&guild_id)
        } else {
            None
        }
    }

    pub fn new_player(&mut self, session_id: String, guild_id: String) -> Result<&Player, AppError> {
        let session = self.sessions.get_mut(&session_id);

        if let Some(s) = session {
            let player = Player::new(
                guild_id.clone(),
                None, 
                100, 
                false, 
                PlayerState::new_empty(),
                None
            );

            s.add_player(player)?;
            
            return Ok(s.players.get(&guild_id).expect("Player should exist."))
        }

        Err(AppError::NotFound(String::from("Could not find session.")))
    }

    pub async fn delete_player(&mut self, session_id: String, guild_id: String) -> Result<(), AppError> {
        let session = self.sessions.get_mut(&session_id);

        if let Some(s) = session {
            s.delete_player(guild_id).await?;
            
            return Ok(())
        }

        Err(AppError::NotFound(String::from("Could not find session.")))
    }

}

pub struct Config {
    pub host: String,
    pub port: u16,
    pub password: SecretString,
}

impl Config {
    pub fn new(host: String, port: u16, password: String) -> Config {
        Config {
            host,
            port,
            password: SecretString::new(password.into()),
        }
    }

    pub fn from_env() -> Config {
        from_filename(".env").ok();
        Config::new(
            std::env::var("HOST")
                .expect("HOST environment variable must be set")
                .into(),
            std::env::var("PORT")
                .expect("PORT environment variable must be set")
                .parse()
                .expect("PORT must be an integer between 0 and 65535"),
            //.into::<u16>(),
            std::env::var("PASSWORD")
                .expect("HOST environment variable must be set")
                .into(),
        )
    }
}
