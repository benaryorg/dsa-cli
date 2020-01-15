pub mod error;
mod action;
mod hero;

use error::*;
use action::Action;
use hero::Hero;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use clap::{App, Arg, ArgMatches, SubCommand};

fn app() -> App<'static,'static>
{
	App::new("dsa-cli")
		.version("0")
		.author("benaryorg <binary@benary.org>")
		.about("Calculates DSA Rolls")
		.setting(clap::AppSettings::SubcommandRequiredElseHelp)
}

fn main() -> Result<()>
{
	let subcommands =
		[ Box::new(action::Dump) as Box<dyn Action>
		, Box::new(action::Cli) as Box<dyn Action>
		, Box::new(action::Roll) as Box<dyn Action>
		];
	let subcommands = subcommands.into_iter()
		.map(|command|
		{
			(command.usage().get_name().to_owned(),command)
		})
		.collect::<HashMap<_,_>>()
		;

	let matches = app()
		.arg
			( Arg::with_name("hero")
			.short("f")
			.long("file")
			.value_name("FILE")
			.help("the XML file for your hero")
			.takes_value(true)
			.required(true)
			)
		.subcommands(subcommands.values().map(|command| command.usage()))
		.get_matches();

	let hero =
	{
		let mut text = String::new();
		let hero = matches.value_of("hero").unwrap();
		let mut file = File::open(hero).chain_err(|| "loading hero file")?;
		file.read_to_string(&mut text)?;
		text.parse::<Hero>().chain_err(|| "failed parsing hero file")?
	};

	let (command, args) = matches.subcommand();
	// we only add subcommands from that hashmap so it MUST be present
	let command = subcommands.get(command).unwrap_or_else(|| unreachable!());
	// we used .subcommand() so the command MUST be present
	let args = args.unwrap_or_else(|| unreachable!());
	if let Some(output) = command.call(&hero,&args)?
	{
		println!("{}",output);
	}

	Ok(())
}

