use dsa::error::*;
use dsa::output;
use dsa::commands;
use dsa::Action;
use dsa::Hero;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use clap::Arg;
use clap::ArgEnum;

fn main() -> Result<()>
{
	let subcommands = vec!
		[ Box::new(commands::Dump) as Box<dyn Action>
		, Box::new(commands::Cli) as Box<dyn Action>
		, Box::new(commands::Roll) as Box<dyn Action>
		];
	let mut subcommands: HashMap<String,Box<dyn Action>> = subcommands.into_iter()
		.map(|command|
		{
			(command.usage().get_name().to_owned(),command)
		})
		.collect();

	let matches = dsa::app()
		.arg
			( Arg::new("hero")
			.short('f')
			.long("file")
			.value_name("FILE")
			.env("DSACLI_FILE")
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

	let formatter: Box<dyn output::Formatter> = matches.value_of("format").map(|format| output::Format::from_str(format, true)).unwrap().unwrap().into();

	let (command, args) = matches.subcommand().unwrap();
	// we only add subcommands from that hashmap so it MUST be present
	let command = subcommands.get_mut(command).unwrap_or_else(|| unreachable!());
	for result in command.call(&hero, args)?.into_iter()
		.map(|result| formatter.format(&result))
	{
		println!("{}",result);
	}

	Ok(())
}

