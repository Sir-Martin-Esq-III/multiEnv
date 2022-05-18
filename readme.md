# MultiEnv

## A CLI tool to manage environment variables across your system!

## Prerequisites

- [Rust build tools](https://www.rust-lang.org/)

## Setup

1. Clone this repo
2. Run `make build` from the root of the project
3. Add `alias multiEnv="~/.multienv/multiEnv"` to your `(bash/zsh)rc` file and reload your terminal
4. Enjoy!

## Commands

- `multiEnv -h` -Display Help text
- `multiEnv -a` -Add a project path from your list of paths you want to update (with no trailing /)
- `multiEnv -r` -Remove a project path from your list of paths you want to update (with no trailing /)
- `multiEnv -p` -Push changes to environment variables across your system
- `multiEnv -d` -Remove existing environment variables across your system
