use super::*;

use std::collections::HashMap;

use rustyline::Editor;

pub struct Cli;

impl Action for Cli
{
	fn usage(&self) -> App
	{
		SubCommand::with_name("cli")
			.about("interactive command line client")
	}

	fn call(&self,hero: &Hero,_: &ArgMatches) -> Result<Option<String>>
	{
		let subcommands =
			[ Box::new(Dump) as Box<dyn Action>
			, Box::new(Roll) as Box<dyn Action>
			];
		let subcommands = subcommands.into_iter()
			.map(|command|
			{
				(command.usage().get_name().to_owned(),command)
			})
			.collect::<HashMap<_,_>>()
			;

		let mut rl = Editor::<()>::new();
		for line in rl.iter("% ")
		{
			let args = line.map(|line|
			{
				line
					.split_whitespace()
					.map(|s| s.to_owned())
					.collect::<Vec<_>>()
			});
			let args = match args
			{
				Ok(ref args) if args.eq(&["exit"]) => break,
				args => args,
			};

			let result = (|args: Result<Vec<_>>|
			{
				let app = crate::app()
					.subcommands(subcommands.values().map(|command| command.usage()));

				// hackily insert an empty string as argv[0]
				let matches = app.get_matches_from_safe([String::new()].into_iter().chain(args?.iter()))?;
				
				let (command, args) = matches.subcommand();
				// we only add subcommands from that hashmap so it MUST be present
				let command = subcommands.get(command).unwrap_or_else(|| unreachable!());
				// we used .subcommand() so the command MUST be present
				let args = args.unwrap_or_else(|| unreachable!());
				command.call(&hero,&args)
			})(args.map_err(|err| err.into()));

			match result
			{
				Ok(None) => {},
				Ok(Some(text)) => println!("{}",text),
				Err(error) => eprintln!("{}",error.description()),
			}
		}

		Ok(None)
	}
}

