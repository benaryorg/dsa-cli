use crate::error::*;
use crate::hero::{BaseValue, Hero};

use rustyline::Editor;
use rand::distributions::Uniform;
use clap::{Arg, App, AppSettings, SubCommand};

pub fn run_cli(hero: Hero) -> Result<()>
{
	let mut current_health = hero.health;
	let mut current_astral = hero.astral;
    let d20: Uniform<_> = (1..21).into();
    let mut rng = rand::thread_rng();
	let sleep_duration = std::time::Duration::from_millis(10);

	let mut rl = Editor::<()>::new();


	let roll_skill = |skill: &str,bonus: &str| {};
	let change_stat = |stat: &mut isize, amount: &str| -> Result<bool>
	{
		*stat = stat.saturating_sub(amount.parse::<isize>()?);
		Ok(false)
	};

	for line in rl.iter("% ")
	{
		let result: Result<bool> = (|line: Result<String>|
		{
			let line = line?;
			let split = line.split_whitespace();
			let mut args = vec![""];
			args.extend(split);
			let matches = crate::app()
				.subcommand
					( SubCommand::with_name("exit")
					.about("interactive mode")
					)
				.get_matches_from_safe(args.iter())?;
			
			if let Some(_matches) = matches.subcommand_matches("exit")
			{
				return Ok(true);
			}
			if let Some(_matches) = matches.subcommand_matches("dump")
			{
				println!("{:#?}",hero);
				return Ok(false);
			}

			Ok(false)
		})(line.map_err(|err| err.into()));

		match result
		{
			Ok(true) => break,
			Ok(false) => {},
			Err(error) =>
			{
				println!("{}",error.description());
			}
		}
	}

	Ok(())
}

