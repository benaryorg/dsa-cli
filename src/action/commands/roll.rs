use super::*;

use std::io::Write;

use rand::distributions::{Distribution, Uniform};

pub struct Roll;

impl Action for Roll
{
	fn usage(&self) -> App
	{
		SubCommand::with_name("roll")
			.about("roll for a skill")
			.arg
				( Arg::with_name("skill")
				.value_name("SKILL")
				.help("the skills to test")
				.takes_value(true)
				.required(true)
				.multiple(true)
				)
	}

	fn call(&self, hero: &Hero, matches: &ArgMatches) -> Result<Option<String>>
	{
		let mut output = Vec::new();

		let d20: Uniform<_> = (1..21).into();
		let mut rng = rand::thread_rng();

		for skill in matches.values_of("skill").unwrap()
		{
			// TODO: use custom error
			let (mut base,values) = hero.skills.get(&skill.to_lowercase()).ok_or(format!("unknown skill '{}'", skill))?;
			let values = values.into_iter().map(|value| Ok((value,hero.basevalues.get(value).ok_or("cannot roll unknown basevalue")?))).collect::<Result<Vec<_>>>()?;
			let rolls = d20.sample_iter(&mut rng).take(3).collect::<Vec<_>>();
			if rolls.len() != 3
			{
				bail!("rng sampling is broken");
			}
			let num_20 = rolls.iter().filter(|i| **i == 20).count();
			let num_1 = rolls.iter().filter(|i| **i == 1).count();
			for roll in rolls.iter().zip(values.iter())
			{
				let stat_name = format!("{:?}",(roll.1).0);
				let die = roll.0;
				let stat = (roll.1).1;
				let result = stat - die;
				writeln!(output,"{:32}: {:3}s - {:3}d = {:3} | {:3} | {:3}", stat_name, stat, die, result, base, base + 0.min(result))?;
				base += 0.min(result);
			}
			if num_20 > 1
			{
				writeln!(output,"\ncritical fail")?;
				continue;
			}
			if num_1 > 1
			{
				writeln!(output,"\ncritical success")?;
				continue;
			}
			if base < 0
			{
				writeln!(output,"\nfailed ({})",base)?;
			}
			else
			{
				writeln!(output,"\nsuccess ({})",base)?;
			}
		}
		Ok(Some(String::from_utf8(output)?.trim_end_matches('\n').to_owned()))
	}
}

