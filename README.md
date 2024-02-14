# mcstat

Command-line utility to ping minecraft servers \
NOTE: May not work with newer servers that reply with JSON MOTDs

## Compiling
`cargo build`

## Usage
`mcstat [OPTIONS] <HOST> <PORT>`

**Arguments:**
- `HOST`  ip of a minecraft server
- `PORT`  port of said server [default: 25565]

**Options:**
- `--favicon`              Shows server favicon in terminal (uses [termimage](https://github.com/nabijaczleweli/termimage)) 
- `--player-sample`        Shows list of players online (if there are any)
- `--size <SIZE>`          Size of favicon in characters
- `-h`, `--help`           Print help
- `-V`, `--version`        Print version
