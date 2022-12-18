use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SteamFriendsListResponse {
    #[serde(rename="friendslist")]
    pub friends_list: SteamFriendsList,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SteamFriendsList {
    pub friends: Vec<SteamFriend>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SteamFriend {
    #[serde(rename="steamid")]
    pub steam_id: String,
    pub relationship: String,
    pub friend_since: u32
}