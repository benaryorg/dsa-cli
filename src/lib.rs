//! A tool aiming to provide a cli client for [DSA (Das Schwarze Auge)](https://en.wikipedia.org/wiki/The_Dark_Eye).
//!
//! It provides the ability to parse the [Heldensoftware](https://www.helden-software.de/) exports and a subset of character related mechanics like:
//! - rolling the dice for you
//! - dumping your character
//! - keeping track of your health, astral points, and stamina
// -Werror in test mode
#![cfg_attr(test, deny(warnings))]

pub mod error;
pub mod output;
mod action;
mod hero;

pub use action::{commands, Action};
pub use hero::{Quality, Hero};

/// Helper to create the basic app with options common between the cli-subcommand and the main app.
///
/// # Examples
///
/// ```
/// # use dsa::app;
/// let matches = app().try_get_matches_from(&["dsa-cli","-o","json","-V"]);
/// assert_eq!(matches.err().unwrap().kind(),clap::ErrorKind::DisplayVersion);
/// ```
pub fn app() -> clap::Command<'static>
{
	use clap::ArgEnum;

	clap::Command::new("dsa-cli")
		.version("0.3.0")
		.author("benaryorg <binary@benary.org>")
		.about("Calculates DSA Rolls")
		.arg_required_else_help(true)
		.arg
			( clap::Arg::new("format")
			.short('o')
			.long("output")
			.alias("format")
			.value_name("FORMAT")
			.help("output format")
			.possible_values(output::Format::value_variants().iter().filter_map(ArgEnum::to_possible_value))
			.default_value("human-readable")
			.ignore_case(true)
			)
}

