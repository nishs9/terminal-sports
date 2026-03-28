# Terminal Sports

Terminal Sports is a Rust-based TUI that you can use to look at live sports scores and statistics. I am building this as a personal project to help myself learn Rust.

For the initial version, I will be focusing on baseball scores (MLB + WBC). However, if everything goes as I envision it, it should be relatively easy to extend this to other major sports like CBB, NBA, etc.

## Current Features
- View live MLB + WBC scores/results (updated every 10 seconds by default)
- View pre-game details about matchups (odds, expected starters, etc.)
- View in-game details about matchups (current pitcher-batter matchup, baserunners, etc.)

## Planned Features
- View play by play data for indidual games
- View box score data for ongoing and completed games
- View even more details about individual MLB + WBC matchups (pre-game, in-game, and post-game)
- Support more sports (NBA, NFL, etc.)
- Users can run the service headlessly and subscribe to an alerts system for games of their choosing

## Data Sources
- ESPN Public API (add link here)

## Getting Started

You will obviously need to install Rust in order to work with this project. I will assume you can figure out how to do that.

Once you have Rust, setup the project as you normally would:

```
cargo install
cargo build
cargo run
```

You will definitely get some compiler warnings but running those commands should pull up the scores UI within your terminal.

You can also run the service using the command `sports` if you install it to your system path:

```
cargo install --path .
sports
```
