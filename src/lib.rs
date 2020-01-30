//! A tool aiming to provide a cli client for [DSA (Das Schwarze Auge)](https://en.wikipedia.org/wiki/The_Dark_Eye).
//!
//! It provides the ability to parse the [Heldensoftware](https://www.helden-software.de/) exports and a subset of character related mechanics like:
//! - rolling the dice for you
//! - dumping your character
//! - keeping track of your health, astral points, and stamina

pub mod error;
pub mod action;
pub mod hero;
pub mod output;

/// Helper to create the basic app with options common between the cli-subcommand and the main app.
///
/// # Examples
///
/// ```
/// let matches = app().get_matches_from(&["dsa-cli","-o","json"]);
/// assert_eq!(matches.value_of("format"),"json");
/// ```
pub fn app() -> clap::App<'static,'static>
{
	clap::App::new("dsa-cli")
		.version("0.1.2")
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
