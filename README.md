![release](https://github.com/benaryorg/dsa-cli/workflows/release/badge.svg)
![ci](https://github.com/benaryorg/dsa-cli/workflows/ci/badge.svg)
[![documentation](https://github.com/benaryorg/dsa-cli/workflows/docs/badge.svg)](https://benaryorg.github.io/dsa-cli/dsa/index.html)

# What is this?

We've got a party of around half a dozen people and I got tired of keeping
track of my health and astral points all the time so I thought I could make a
command-line tool.
There is a neat *cli* subcommand which keeps track of your health, stamina, and
astral points via the command line interface, offering you a history and so on.
In most of your rolls you will have to account for special stuff (fatigue,
perks, etc.) yourself for now.

Furthermore the ruleset we use is a simplified version of DSA 4.1.
Don't expect too much from this tool, it'll be highly customized.

# How to build/use it?

Building is done via *cargo* since this is a Rust project.
Most of the names and stuff are taken from the (German \*shudder\*) XML export
of the [Heldensoftware](https://www.helden-software.de/).
You can roll for something like that:

```bash
cargo run -q -- --file helden-software-export.xml roll wettervorhersage
```

## Documentation

You can find the documentation for the current master on the [GitHub
pages](https://benaryorg.github.io/dsa-cli/dsa/index.html) for this project.

## Automation

If you aren't into the *cli* thing you can always either adapt the code to give
it a UI you prefer, or you can build around its CLI and use the JSON output to
build a wrapper around it.

# Will it have TUI?

Commit
[*477abed*](https://github.com/benaryorg/dsa-cli/commit/477abedca0b561a6aee8b67ceaa106d4031e4523)
had some nice gauges and so on for tracking that, but I concentrated on making
rolls early on to put this thing to actual use.

You can always use *main.rs* and *cli.rs* as an example on what *can* be done
with this tool so you could build a TUI or even GUI around what's there already.

