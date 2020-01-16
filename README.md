[![github-ci-badge](https://github.com/benaryorg/dsa-cli/workflows/ci/badge.svg)](https://github.com/benaryorg/dsa-cli/actions)

# What is this?

We've got a party of around half a dozen people and I got tired of keeping
track of my health and astral points all the time so I thought I could make a
command-line tool.
Commit
[*477abed*](https://github.com/benaryorg/dsa-cli/commit/477abedca0b561a6aee8b67ceaa106d4031e4523)
has some nice gauges and so on for tracking that, but I concentrated on making
rolls early on to put this thing to actual use.

# How to build/use it?

Building is done via *cargo* since this is a Rust project.
Most of the names and stuff are taken from the (German \*shudder\*) XML export
of the [Heldensoftware](https://www.helden-software.de/).
You can roll for something like that:

```bash
cargo run -q -- --file helden-software-export.xml roll wettervorhersage
```

Or use the `cli` subcommand which I am hoping to improve far enough to have
some nice output of your current health status and so on.

In most of your rolls you will have to account for special stuff (fatigue,
perks, etc.) yourself for now.

Furthermore the ruleset we use is a *very* simplified version of DSA 4.1.
Don't expect too much from this tool, it'll be highly customized.

