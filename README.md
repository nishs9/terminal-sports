# Terminal Sports

Terminal Sports is a Rust-based TUI that you can use to look at live sports scores and statistics. This is a Rust-based tool that I am building as a little personal project to help myself learn Rust. Depending on how it goes will directly affect how much polish I decide to apply to the project.

For the MVP, I will be focusing on baseball scores (MLB + WBC). However, if everything goes as I envision it, it should be relatively easy to extend this to other major sports like CBB, NBA, etc.`

## Planned Features
- View MLB + WBC live scores/results
- View details about individual MLB + WBC matchups
- Users can run the service headlessly and subscribe to an alerts system for games of their choosing
- View play by play data for indidual games
- View box score data for ongoing and completed games

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
