# Terminal Sports

Terminal Sports is a Rust-based TUI that you can use to look at live sports scores and statistics. I am building this as a personal project to help myself learn Rust.

For the initial version, I will be focusing on baseball scores (MLB + WBC). However, if everything goes as I envision it, it should be relatively easy to extend this to other major sports like CBB, NBA, etc.

## Current Features
- View live MLB + WBC scores/results (updated every 10 seconds by default)
- View pre-game details such as odds, expected starters, etc.
- View in-game details such as current pitcher-batter matchup, baserunners, last play, etc.
- View post-game details such as winning/losing pitcher, final score + linescore, etc.

## Planned Features
- Support more sports (NBA, NFL, etc.)
- View play by play data for indidual games
- View box score data for ongoing and completed games
- Users can run the service headlessly and subscribe to an alerts system for games of their choosing

## Data Sources
- [ESPN Public API](https://github.com/pseudo-r/Public-ESPN-API)

I'm not necessarily using the webserver that is at the repo above, but I am calling the same underlying ESPN API endpoints.

## Getting Started

You will obviously need to install Rust in order to work with this project. I will assume you can figure out how to do that.

Once you have Rust, setup the project as you normally would:

```
cargo build
cargo run
```

You will definitely get some compiler warnings but running those commands should pull up the scores UI within your terminal.