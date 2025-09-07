# Filter Maker
An item filter generator for Path of Exile.

## Getting Started
1. Use `cargo run` or run the `update.ps1` script to generate your filter
1. Edit `config/filter.toml` however you see fit
1. Use `cargo run` or run the `update.ps1` script to regenerate your filter

## Options
- Run `./update.ps1 -Latest` to replace your local filter with the latest example (this will **not** replace your `configs.destination`)
- Run `./update.ps1 -Poe2` to generate a filter for Path of Exile 2
- Run `./update.ps1 -Poe2 -Latest` to replace your local Path of Exile 2 filter with the latest example