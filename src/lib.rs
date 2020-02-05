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
pub use hero::{BaseValue, Hero};

/// Helper to create the basic app with options common between the cli-subcommand and the main app.
///
/// # Examples
///
/// ```
/// # use dsa::app;
/// let matches = app().get_matches_from_safe(&["dsa-cli","-o","json","-V"]);
/// assert_eq!(matches.err().unwrap().kind,clap::ErrorKind::VersionDisplayed);
/// ```
pub fn app() -> clap::App<'static,'static>
{
	clap::App::new("dsa-cli")
		.version("0.2.1")
		.author("benaryorg <binary@benary.org>")
		.about("Calculates DSA Rolls")
		.setting(clap::AppSettings::SubcommandRequiredElseHelp)
		.arg
			( clap::Arg::with_name("format")
			.short("o")
			.long("output")
			.alias("format")
			.value_name("FORMAT")
			.help("output format")
			.possible_values(&output::Format::variants())
			.default_value("humanreadable")
			.case_insensitive(true)
			)
}

