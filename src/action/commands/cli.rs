use super::*;
use crate::app;
use crate::output;
use crate::output::Output;

use std::collections::HashMap;

use rustyline::{Config, Editor};

pub struct Cli;

// TODO: more docs and examples
/// Boots up a cli interface with trackers holding mutable state.
impl Action for Cli
{
	fn usage<'a,'b>(&'a self) -> Command<'b>
	{
		Command::new("cli")
			.about("interactive command line client")
	}

	fn call(&mut self,hero: &Hero,_: &ArgMatches) -> Result<Vec<Output>>
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
			let words = line
				.map_err(|err| err.into())
				.and_then(|line| Ok(shell_words::split(&line)?));

			if let Ok(ref args) = words
			{
				if args.eq(&["exit"])
				{
					break;
				}
			}

			let result: Result<Vec<_>> = words
				// build the clap Command
				.and_then(|words|
				{
					let app = app().subcommands(subcommands.values().map(|command| command.usage()));
					// hackily insert an empty string as argv[0]
					Ok(app.try_get_matches_from(std::iter::once(String::new()).chain(words))?)
				})
				.and_then(|matches|
				{
					// get the corresponding subcommand
					let (command, args) = matches.subcommand().unwrap();
					// we only add subcommands from that hashmap so it MUST be present
					let command = subcommands.get_mut(command).unwrap_or_else(|| unreachable!());

					let formatter: Box<dyn output::Formatter> = matches.value_of("format").map(|format| output::Format::from_str(format, true)).unwrap().unwrap().into();

					let result = command.call(hero, args)?.into_iter()
						.map(|result| formatter.format(&result))
						.collect();

					Ok(result)
				});

			match result
			{
				Ok(outputs) =>
				{
					for output in outputs
					{
						println!("{}",output)
					}
				},
				Err(error) => eprintln!("{}", error.description()),
			}
		}

		Ok(vec![])
	}
}

