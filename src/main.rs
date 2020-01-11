#[macro_use] extern crate error_chain;
extern crate roxmltree;

pub mod error;
mod hero;

use error::*;
use hero::Hero;

use std::fs::File;
use std::io::Read;

use clap::{Arg, App, SubCommand};

fn main() -> Result<()>
{
	let matches = App::new("dsa-cli")
		.version("0")
		.author("benaryorg <binary@benary.org>")
		.about("Calculates DSA Rolls")
			.arg
				( Arg::with_name("hero")
				.short("f")
				.long("file")
				.value_name("FILE")
				.help("the XML file for your hero")
				.takes_value(true)
				.required(true)
				)
		.subcommand
			( SubCommand::with_name("dump")
			.about("dump hero information")
			)
		.subcommand
			( SubCommand::with_name("cli")
			.about("interactive mode")
			)
		.get_matches();

	let hero =
	{
		let mut text = String::new();
		let hero = matches.value_of("hero").unwrap();
		let mut file = File::open(hero).chain_err(|| "loading hero file")?;
		file.read_to_string(&mut text)?;
		text.parse::<Hero>().chain_err(|| "failed parsing hero file")?
	};

	if let Some(_matches) = matches.subcommand_matches("dump")
	{
		eprintln!("{:#?}",hero);
		return Ok(());
	}

	if let Some(_matches) = matches.subcommand_matches("cli")
	{
		unimplemented!();
	}

	Ok(())
}
