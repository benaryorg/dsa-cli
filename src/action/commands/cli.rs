use super::*;
use crate::output;
use crate::output::Output;

use std::collections::HashMap;

use rustyline::{Config, Editor};

pub struct Cli;

impl Action for Cli
{
	fn usage<'a,'b>(&'a self) -> App<'b,'b>
	{
		SubCommand::with_name("cli")
			.about("interactive command line client")
	}

	fn call(&mut self,hero: &Hero,_: &ArgMatches) -> Result<Output>
	{
		let subcommands = vec!
			[ Dump::new_action()
			, Roll::new_action()
			, Tracker::new_action("health",hero.health,hero.health)
			, Tracker::new_action("astral",hero.astral,hero.astral)
			, Tracker::new_action("stamina",hero.stamina,hero.stamina)
			];
		let mut subcommands: HashMap<String,Box<dyn Action>> = subcommands.into_iter()
			.map(|command|
			{
				(command.usage().get_name().to_owned(),command)
			})
			.collect();

		let mut rl = Editor::<()>::with_config(Config::builder()
			.max_history_size(1024*512) // with 80 characters per line that's 40MiB
			.history_ignore_dups(false)
			.history_ignore_space(true)
			.auto_add_history(true)
			.tab_stop(4)
			.build());
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

			let result = (|args: Result<Vec<_>>| -> Result<Option<String>>
			{
				let app = crate::app()
					.subcommands(subcommands.values().map(|command| command.usage()));

				// hackily insert an empty string as argv[0]
				let matches = app.get_matches_from_safe([String::new()].iter().chain(args?.iter()))?;
				
				let (command, args) = matches.subcommand();
				// we only add subcommands from that hashmap so it MUST be present
				let command = subcommands.get_mut(command).unwrap_or_else(|| unreachable!());
				// we used .subcommand() so the command MUST be present
				let args = args.unwrap_or_else(|| unreachable!());

				let formatter = output::humanreadable();

				let result = command.call(&hero,&args)?;
				Ok(formatter.format(&result))
			})(args.map_err(|err| err.into()));

			match result
			{
				Ok(None) => {},
				Ok(Some(text)) => println!("{}",text),
				Err(error) => eprintln!("{}",error.description()),
			}
		}

		Ok(Output::None)
	}
}

