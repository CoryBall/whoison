use clap::Args;

pub mod steam_friend;
pub mod steam_player_summary;

use crate::steam::steam_friend::{SteamFriendsListResponse, SteamFriend};
use crate::steam::steam_player_summary::{SteamPlayerSummaryResponse, SteamPlayerSummary};
use crate::online_friend::OnlineFriend;

#[derive(Args, Debug)]
pub struct SteamArgs {
    // Your Steam API Key
    #[arg(long, env = "STEAM_API_KEY")]
    pub steam_api_key: String,
    /// Your steam Id
    #[arg(long, env = "STEAM_ID")]
    pub steam_id: String,
}

pub struct SteamClient {
    pub steam_api_key: String,
    pub steam_id: String,
}

impl SteamClient {
    async fn get_friends(&self) -> Result<Vec<SteamFriend>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let query = vec![
            ("key", &self.steam_api_key),
            ("steamid", &self.steam_id)
        ];

        let friends_list_url = "http://api.steampowered.com/ISteamUser/GetFriendList/v0001/";

        let response = client.get(friends_list_url)
            .query(&query)
            .send()    
            .await?
            // each response is wrapped in a `Result` type
            // we'll unwrap here for simplicity
            .json::<SteamFriendsListResponse>()
            .await?;

        Ok(response.friends_list.friends)
    }

    async fn get_friend_status(&self, steam_friends: Vec<SteamFriend>) -> Result<Vec<SteamPlayerSummary>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let mut friend_ids: Vec<String> = vec![];

        for friend in steam_friends {
            friend_ids.push(friend.steam_id)
        }

        let steam_ids = &friend_ids.join(",");

        let query = vec![
            ("key", &self.steam_api_key),
            ("steamids", steam_ids)
        ];

        let player_summaries_url = "http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/";

        let response = client.get(player_summaries_url)
            .query(&query)
            .send()    
            .await?
            // each response is wrapped in a `Result` type
            // we'll unwrap here for simplicity
            .json::<SteamPlayerSummaryResponse>()
            .await?;

            Ok(response.friends_list.friends)
    }

    pub async fn get_online_friends(&self) -> Result<Vec<OnlineFriend>, Box<dyn std::error::Error>> {
        let steam_friends = SteamClient::get_friends(self).await?;
        let friend_statuses = SteamClient::get_friend_status(self, steam_friends).await?;

        let mut online_friends: Vec<OnlineFriend> = vec![];

        for steam_friend in friend_statuses {
            match steam_friend.profile_state {
                Some(_) =>
                    match steam_friend.game {
                        Some(game) => 
                            online_friends.push(OnlineFriend {
                                friend_name: String::from(steam_friend.player_name),
                                platform: String::from("Steam"),
                                game: String::from(game)
                            }),
                        None => continue,
                    }
                None => continue,
            }
        }

        Ok(online_friends)
    }
}