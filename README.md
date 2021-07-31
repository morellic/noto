# noto
A simple command line tool for note management and browsing.

The current goal of the project is getting familiar with rust.

## Usage
See `cargo run -- --help` (or via the executable `noto --help`): 
```
USAGE:
    noto <action-type> [path]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <action-type>     [possible values: Find, Grep, Ls]      
    <path>
```

## Features
- `noto ls <path>`

## todo
- grep
- find
- cat
- vim note editing and vieing
- note editing and viewing with other text editors? e.g. configurable
- fancier note searching?
- note tags?
