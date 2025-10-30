# ⚡ Thop - Tmux/Multiplexer Hopper
Fast and lightweight interactive CLI for defining and jumping between projects / pre-defined multiplexer sessions.

## About
Light and quick to use way of managing multiplexer sessions

### Features:
- Fast navigation to desired project / session from anywhere (including from inside of a session)
- Easy to edit universal yaml templates
- Execute shell commands in all/desired windows/panes
- Tmux support

## Dependencies
- [fzf](https://github.com/junegunn/fzf)
- [tmux](https://github.com/tmux/tmux) 1.8+ (except for 2.5)

## Building
`cargo build`

## Running
`cargo run [command] [args]`

## Testing
`cargo test`

## Current state
This project follows [Semantic Versioning](https://semver.org/), but currently it's at version v0 as it's in development.
Destination is set but things can still change and break backwards compatibility; That includes templates they are not getting migrations until v1.

### TODO's for V1:
- [x] CLI library evaluation and integration (Clap)
- [x] Minimum viable template definition
- [ ] V1 ready template definition
- [x] Template creation
- [x] Template opening
- [x] Template removing
- [ ] Template editing
- [ ] Template killing
- [ ] Storing templates in file system
- [x] Setup architecture
- [x] Unit tests setup
- [x] Integration tests setup
- [ ] Architecture tests
- [x] Rust CI setup
- [x] Automate dependency updates (Renovate)
- [ ] Sonar coverage reports
- [ ] Release model
- [ ] Install script
- [ ] Tmux engine integration
- [ ] Template migrations

### Post V1 Ideas:
- Generic config file
- Zellij engine integration
- Video showcase in README
- Setup and Teardown configuration
- Own interactive selection picker (instead of relying on fzf)
