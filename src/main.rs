use dotenv::dotenv;
use clap::Parser;

mod steam;
mod online_friend;

use crate::steam::SteamClient;


#[derive(clap::Parser, Debug)]
struct Args {
    /// Name of the platform to search friends on ("steam", "all")
    #[arg(short = 'p', long, default_value = "all")]
    platform: String,
    #[command(flatten)]
    steam: steam::SteamArgs,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let args = Args::parse();


    let steam_client = SteamClient {
        steam_api_key: args.steam.steam_api_key,
        steam_id: args.steam.steam_id,
    };

    let steam_friends = steam_client.get_online_friends().await?;
    
    let online_friends = [steam_friends].concat();

    for friend in online_friends {
        println!("{} is on {} playing {}", friend.friend_name, friend.platform, friend.game);
    }

    Ok(())
}
