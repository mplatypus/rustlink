use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    encoded: String,
    info: TrackInfo,
    plugin_data: HashMap<String, String>,
    user_data: HashMap<String, String>,
}

impl Track {
    pub fn new(
        encoded: String,
        info: TrackInfo,
        plugin_data: HashMap<String, String>,
        user_data: HashMap<String, String>,
    ) -> Track {
        Track {
            encoded,
            info,
            plugin_data,
            user_data,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TrackInfo {
    identifier: String,
    is_seekable: bool,
    author: String,
    length: usize,
    is_stream: bool,
    position: usize,
    title: String,
    uri: Option<String>,
    artwork_url: Option<String>,
    isrc: Option<String>,
    source_name: Option<String>,
}

impl TrackInfo {
    pub fn new(
        identifier: String,
        is_seekable: bool,
        author: String,
        length: usize,
        is_stream: bool,
        position: usize,
        title: String,
        uri: Option<String>,
        artwork_url: Option<String>,
        isrc: Option<String>,
        source_name: Option<String>,
    ) -> TrackInfo {
        TrackInfo {
            identifier,
            is_seekable,
            author,
            length,
            is_stream,
            position,
            title,
            uri,
            artwork_url,
            isrc,
            source_name,
        }
    }
}
