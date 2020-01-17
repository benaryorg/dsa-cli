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
				( Arg::with_name("modifier")
				.short("m")
				.long("modifier")
				.alias("mod")
				.help("modification as positive (bad) or negative (good) integer")
				.takes_value(true)
				.multiple(true)
				.number_of_values(1)
				)
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

		let mods = matches.values_of("modifier")
			.map(|mods| mods
				.map(|modi| Ok(modi.parse()?))
				.collect::<Result<Vec<isize>>>()
				.map(|mods| mods.into_iter().sum())
			)
			.unwrap_or(Ok(0))?;

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
		let result = 0.max(base - mods) + values.iter()
			.map(|stat| stat + 0.min(base - mods))
			.zip(rolls.iter())
			.map(|(stat,die)| (stat-die).min(0))
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
			mods: mods,
		})
	}
}

