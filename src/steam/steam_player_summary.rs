use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SteamPlayerSummaryResponse {
    #[serde(rename="response")]
    pub friends_list: SteamPlayerSummaryList,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SteamPlayerSummaryList {
    #[serde(rename="players")]
    pub friends: Vec<SteamPlayerSummary>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SteamPlayerSummary {
    #[serde(rename="steamid")]
    pub steam_id: String,
    #[serde(rename="personaname")]
    pub player_name: String,
    #[serde(rename="profilestate", skip_serializing_if = "Option::is_none")]
    pub profile_state: Option<u8>,
    #[serde(rename="gameextrainfo", skip_serializing_if = "Option::is_none")]
    pub game: Option<String>,
}