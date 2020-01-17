use super::*;

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
				)
	}

	fn call(&self, hero: &Hero, matches: &ArgMatches) -> Result<Output>
	{
		let d20: Uniform<_> = (1..21).into();
		let mut rng = rand::thread_rng();

		let skill = matches.value_of("skill").unwrap();

		// TODO: use custom error
		let (base,values_enum) = hero.skills.get(&skill.to_lowercase()).ok_or(format!("unknown skill '{}'", skill))?;

		let values =
		{
			let mut iter = values_enum.iter()
				.map(|value| -> Result<isize>
				{
					Ok(*hero.basevalues.get(value).ok_or("cannot roll unknown basevalue")?)
				});
			[
				iter.next().ok_or("basevalue retrieval is broken")??,
				iter.next().ok_or("basevalue retrieval is broken")??,
				iter.next().ok_or("basevalue retrieval is broken")??,
			]
		};
		let rolls =
		{
			let mut iter = d20.sample_iter(&mut rng);
			[
				iter.next().ok_or("rng sampling is broken")?,
				iter.next().ok_or("rng sampling is broken")?,
				iter.next().ok_or("rng sampling is broken")?,
			]
		};
		let num_20 = rolls.iter().filter(|i| **i == 20).count();
		let num_1 = rolls.iter().filter(|i| **i == 1).count();
		let result = base + rolls.iter().zip(values.iter())
			.map(|(die,stat)| (stat-die).min(0))
			.sum::<isize>();

		Ok(Output::Roll
		{
			success: (num_20 < 2) && (result >= 0 || num_1 > 1),
			critical: (num_20 > 1) || (num_1 > 1),
			dice: rolls,
			checks: values_enum.clone(),
			stat: values,
			remainder: result,
			base: *base,
			// TODO: mods
			mods: 0,
		})
	}
}

