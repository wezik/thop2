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

### TODO's:
- CLI library evaluation and integration
- Template removal
- Template editing
- Template killing
- Storing templates in file system
- Integration tests
- Architecture tests
- Running tests in pipelines
- Sonar coverage reports
- Release workflow
- Tmux engine integration
- Template migrations

### Ideas:
- Generic config file
- Zellij engine integration
- Video showcase in README
- Setup and Teardown configuration
- Own interactive selection picker (instead of relying on fzf)
