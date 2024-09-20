use serde::{Deserialize, Serialize};

use super::{player::PlayerState, track::Track};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum EventType {
    TrackStartEvent,
    TrackEndEvent,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum EventCode {
    Ready,
    PlayerUpdate,
    Stats,
    Event,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReadyEvent {
    op: EventCode,
    resumed: bool,
    session_id: String,
}

impl ReadyEvent {
    pub fn new(resumed: bool, session_id: String) -> ReadyEvent {
        ReadyEvent {
            op: EventCode::Ready,
            resumed,
            session_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerUpdateEvent {
    op: EventCode,
    guild_id: String,
    state: PlayerState,
}

impl PlayerUpdateEvent {
    pub fn new(guild_id: String, state: PlayerState) -> PlayerUpdateEvent {
        PlayerUpdateEvent {
            op: EventCode::PlayerUpdate,
            guild_id,
            state,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TrackStartEvent {
    op: EventCode,
    #[serde(rename = "type")]
    event_type: EventType,
    guild_id: String,
    track: Track,
}

impl TrackStartEvent {
    pub fn new(guild_id: String, track: Track) -> TrackStartEvent {
        TrackStartEvent {
            op: EventCode::PlayerUpdate,
            event_type: EventType::TrackStartEvent,
            guild_id,
            track,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum TrackEndReasonType {
    Stopped,
    Failed,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TrackEndEvent {
    op: EventCode,
    #[serde(rename = "type")]
    event_type: EventType,
    guild_id: String,
    track: Track,
    reason: TrackEndReasonType,
}

impl TrackEndEvent {
    pub fn new(guild_id: String, track: Track, reason: TrackEndReasonType) -> TrackEndEvent {
        TrackEndEvent {
            op: EventCode::PlayerUpdate,
            event_type: EventType::TrackEndEvent,
            guild_id,
            track,
            reason,
        }
    }
}
