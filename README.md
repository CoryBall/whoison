# Who is on?
Looks through various game platforms to see what your friends are playing so you don't have to!

## Usage
Run `whoison --help` shows all platforms along with required arguments
> you can set the respective ENVIRONMENT VARIABLES instead of explicitly passing some arguments

## Supported platforms:
 - Steam
 - More coming soon


## Requirements
This project supports .env files in the project directory for saving environment variables
 - Steam:
   - [API Key](https://steamcommunity.com/dev/apikey): use this with `--steam-api-key` or `STEAM_API_KEY` in the .env file
   - [Your Steam ID](https://store.steampowered.com/account/): use this with `--steam-id` or `STEAM_ID` in the .env file